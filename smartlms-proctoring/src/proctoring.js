/**
 * SmartLMS Proctoring - Browser-side JavaScript Library
 * Handles browser lockdown, event detection, and video capture
 */

(function() {
    'use strict';

    // Configuration
    const DEFAULT_CONFIG = {
        browserLockdown: true,
        fullscreenRequired: true,
        tabSwitchAlerts: true,
        webcamRequired: true,
        screenRecording: false,
        audioRecording: false,
        violationThreshold: 3,
        apiEndpoint: '/api/v1/proctoring'
    };

    class ProctoringClient {
        constructor(config = {}) {
            this.config = { ...DEFAULT_CONFIG, ...config };
            this.sessionId = null;
            this.violationCount = 0;
            this.isRecording = false;
            this.mediaRecorder = null;
            this.videoChunks = [];
            this.webcamStream = null;
            this.screenStream = null;
            
            // Bind methods
            this.handleVisibilityChange = this.handleVisibilityChange.bind(this);
            this.handleFullscreenChange = this.handleFullscreenChange.bind(this);
            this.handleKeyDown = this.handleKeyDown.bind(this);
            this.handleCopy = this.handleCopy.bind(this);
            this.handleContextMenu = this.handleContextMenu.bind(this);
        }

        /**
         * Initialize proctoring for an exam
         */
        async initialize(sessionId, examId) {
            this.sessionId = sessionId;
            console.log('Initializing proctoring for session:', sessionId);

            // Request webcam access if required
            if (this.config.webcamRequired) {
                await this.requestWebcam();
            }

            // Start recording
            await this.startRecording();

            // Setup event listeners
            this.setupEventListeners();

            // Notify backend
            await this.notifyBackend('session_started', {
                sessionId,
                examId,
                userAgent: navigator.userAgent,
                screenResolution: `${screen.width}x${screen.height}`
            });

            console.log('Proctoring initialized');
        }

        /**
         * Request webcam access
         */
        async requestWebcam() {
            try {
                this.webcamStream = await navigator.mediaDevices.getUserMedia({
                    video: {
                        width: { ideal: 1280 },
                        height: { ideal: 720 },
                        facingMode: 'user'
                    },
                    audio: this.config.audioRecording
                });
                
                // Create video element for preview
                const video = document.createElement('video');
                video.srcObject = this.webcamStream;
                video.autoplay = true;
                video.playsInline = true;
                video.muted = true;
                video.style.cssText = 'position:fixed;top:10px;right:10px;width:160px;height:90px;border-radius:8px;box-shadow:0 2px 8px rgba(0,0,0,0.3);z-index:999999;';
                document.body.appendChild(video);
                
                this.webcamVideo = video;
                console.log('Webcam access granted');
            } catch (error) {
                console.error('Webcam access denied:', error);
                this.reportViolation('webcam_denied', 'high', 'Unable to access webcam');
            }
        }

        /**
         * Start video recording
         */
        async startRecording() {
            try {
                // Start webcam recording
                if (this.webcamStream) {
                    this.mediaRecorder = new MediaRecorder(this.webcamStream, {
                        mimeType: 'video/webm;codecs=vp9'
                    });

                    this.mediaRecorder.ondataavailable = (event) => {
                        if (event.data.size > 0) {
                            this.videoChunks.push(event.data);
                        }
                    };

                    this.mediaRecorder.start(10000); // Capture in 10-second chunks
                    this.isRecording = true;
                    console.log('Recording started');
                }

                // Start screen recording if enabled
                if (this.config.screenRecording) {
                    await this.startScreenRecording();
                }
            } catch (error) {
                console.error('Failed to start recording:', error);
            }
        }

        /**
         * Start screen recording
         */
        async startScreenRecording() {
            try {
                this.screenStream = await navigator.mediaDevices.getDisplayMedia({
                    video: {
                        cursor: 'always'
                    },
                    audio: false
                });

                // Handle when user stops sharing via browser UI
                this.screenStream.getVideoTracks()[0].onended = () => {
                    this.reportViolation('screen_recording_stopped', 'medium', 'Screen recording was stopped');
                };
            } catch (error) {
                console.warn('Screen recording not available:', error);
            }
        }

        /**
         * Stop recording and cleanup
         */
        async stopRecording() {
            if (this.mediaRecorder && this.isRecording) {
                this.mediaRecorder.stop();
                this.isRecording = false;
            }

            // Stop all tracks
            if (this.webcamStream) {
                this.webcamStream.getTracks().forEach(track => track.stop());
            }
            if (this.screenStream) {
                this.screenStream.getTracks().forEach(track => track.stop());
            }

            // Remove video element
            if (this.webcamVideo) {
                this.webcamVideo.remove();
            }

            // Notify backend
            await this.notifyBackend('session_ended', {
                sessionId: this.sessionId,
                violationCount: this.violationCount
            });

            console.log('Recording stopped, violations:', this.violationCount);
        }

        /**
         * Setup event listeners for security
         */
        setupEventListeners() {
            // Tab visibility changes
            document.addEventListener('visibilitychange', this.handleVisibilityChange);

            // Fullscreen changes
            document.addEventListener('fullscreenchange', this.handleFullscreenChange);
            document.addEventListener('webkitfullscreenchange', this.handleFullscreenChange);

            // Keyboard shortcuts
            document.addEventListener('keydown', this.handleKeyDown);

            // Copy/paste
            document.addEventListener('copy', this.handleCopy);
            document.addEventListener('cut', this.handleCopy);
            document.addEventListener('paste', this.handleCopy);

            // Right-click
            document.addEventListener('contextmenu', this.handleContextMenu);
        }

        /**
         * Handle tab visibility changes
         */
        handleVisibilityChange() {
            if (document.hidden && this.config.tabSwitchAlerts) {
                this.violationCount++;
                this.reportViolation('tab_switch', 'medium', 'User switched to another tab');
                
                // Show warning
                this.showWarning('Tab switch detected! This is being recorded.');
            }
        }

        /**
         * Handle fullscreen exit
         */
        handleFullscreenChange() {
            if (!document.fullscreenElement && this.config.fullscreenRequired) {
                this.violationCount++;
                this.reportViolation('fullscreen_exit', 'high', 'User exited fullscreen mode');
                this.showWarning('Fullscreen is required for this exam. Please re-enter fullscreen.');
                
                // Attempt to re-enter fullscreen
                document.documentElement.requestFullscreen().catch(() => {});
            }
        }

        /**
         * Handle keyboard events
         */
        handleKeyDown(event) {
            // Block common shortcuts
            const blockedShortcuts = [
                'Ctrl+Shift+I',   // DevTools
                'Ctrl+Shift+J',   // Console
                'Ctrl+Shift+C',   // Inspector
                'Ctrl+U',         // View Source
                'Ctrl+S',         // Save
                'Ctrl+P',         // Print
                'F12'             // DevTools
            ];

            const key = [];
            if (event.ctrlKey) key.push('Ctrl');
            if (event.shiftKey) key.push('Shift');
            if (event.altKey) key.push('Alt');
            key.push(event.key.toUpperCase());

            const shortcut = key.join('+');
            if (blockedShortcuts.includes(shortcut)) {
                event.preventDefault();
                this.violationCount++;
                this.reportViolation('keyboard_shortcut', 'high', `Attempted shortcut: ${shortcut}`);
            }
        }

        /**
         * Handle copy/paste attempts
         */
        handleCopy(event) {
            event.preventDefault();
            this.violationCount++;
            this.reportViolation('copy_paste', 'medium', 'Copy/paste attempt detected');
            this.showWarning('Copy/paste is disabled during the exam.');
        }

        /**
         * Handle right-click
         */
        handleContextMenu(event) {
            event.preventDefault();
            this.violationCount++;
            this.reportViolation('right_click', 'low', 'Right-click attempt detected');
        }

        /**
         * Report violation to backend
         */
        async reportViolation(type, severity, description) {
            console.log('Violation:', type, severity, description);
            
            const violation = {
                sessionId: this.sessionId,
                violationType: type,
                severity: severity,
                description: description,
                timestamp: new Date().toISOString(),
                violationCount: this.violationCount
            };

            try {
                await this.notifyBackend('violation', violation);
            } catch (error) {
                console.error('Failed to report violation:', error);
            }

            // Check threshold
            if (this.violationCount >= this.config.violationThreshold) {
                this.handleViolationThreshold();
            }
        }

        /**
         * Handle when violation threshold is reached
         */
        handleViolationThreshold() {
            this.showWarning(`Warning: You have reached ${this.config.violationThreshold} violations. Further violations may result in automatic exam termination.`);
            // In production: could auto-submit or alert proctor
        }

        /**
         * Show warning message
         */
        showWarning(message) {
            const warning = document.createElement('div');
            warning.id = 'proctoring-warning';
            warning.innerHTML = `
                <div style="
                    position: fixed;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    background: #ff4444;
                    color: white;
                    padding: 20px 40px;
                    border-radius: 8px;
                    font-size: 16px;
                    z-index: 999999;
                    box-shadow: 0 4px 20px rgba(0,0,0,0.3);
                    text-align: center;
                ">
                    ⚠️ ${message}
                </div>
            `;
            document.body.appendChild(warning);
            
            setTimeout(() => warning.remove(), 5000);
        }

        /**
         * Notify backend of events
         */
        async notifyBackend(eventType, data) {
            try {
                await fetch(`${this.config.apiEndpoint}/events`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        sessionId: this.sessionId,
                        eventType,
                        data
                    })
                });
            } catch (error) {
                console.error('Failed to notify backend:', error);
            }
        }

        /**
         * Cleanup event listeners
         */
        cleanup() {
            document.removeEventListener('visibilitychange', this.handleVisibilityChange);
            document.removeEventListener('fullscreenchange', this.handleFullscreenChange);
            document.removeEventListener('keydown', this.handleKeyDown);
            document.removeEventListener('copy', this.handleCopy);
            document.removeEventListener('contextmenu', this.handleContextMenu);
            
            this.stopRecording();
        }
    }

    // Export to global
    window.SmartLMSProctoring = {
        Client: ProctoringClient,
        create: (config) => new ProctoringClient(config)
    };

})();