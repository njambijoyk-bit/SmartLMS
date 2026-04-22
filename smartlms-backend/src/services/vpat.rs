// Phase 16 Enhancement: VPAT (Voluntary Product Accessibility Template) Generator
use chrono::{Date, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConformanceStatus { 
    Supports, 
    PartiallySupports, 
    DoesNotSupport, 
    NotApplicable 
}

impl ConformanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConformanceStatus::Supports => "Supports",
            ConformanceStatus::PartiallySupports => "Partially Supports",
            ConformanceStatus::DoesNotSupport => "Does Not Support",
            ConformanceStatus::NotApplicable => "Not Applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WcagConformanceLevel { A, AA, AAA }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpatCriterion {
    pub id: Uuid,
    pub report_id: Uuid,
    pub criterion_number: String,
    pub criterion_name: String,
    pub wcag_criterion: Option<String>,
    pub conformance_level: Option<WcagConformanceLevel>,
    pub conformance_status: ConformanceStatus,
    pub user_notes: Option<String>,
    pub evidence_url: Option<String>,
    pub remediation_plan: Option<String>,
    pub target_remediation_date: Option<Date<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpatReport {
    pub id: Uuid,
    pub product_name: String,
    pub product_version: String,
    pub report_date: Date<Utc>,
    pub vendor_name: String,
    pub vendor_contact: Option<String>,
    pub wcag_level: WcagConformanceLevel,
    pub section_508_compliant: bool,
    pub en_301_549_compliant: bool,
    pub overall_compliance_score: f64,
    pub total_criteria: i32,
    pub passed_criteria: i32,
    pub partially_met_criteria: i32,
    pub not_met_criteria: i32,
    pub not_applicable_criteria: i32,
    pub remarks: Option<String>,
    pub criteria: Vec<VpatCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpatGenerationRequest {
    pub product_name: String,
    pub product_version: String,
    pub vendor_name: String,
    pub vendor_contact: Option<String>,
    pub wcag_level: WcagConformanceLevel,
    pub include_section_508: bool,
    pub include_en_301_549: bool,
}

pub struct VpatService;

impl VpatService {
    /// Create a new VPAT report for accessibility compliance documentation
    pub fn create_report(request: VpatGenerationRequest) -> VpatReport {
        let mut report = VpatReport {
            id: Uuid::new_v4(),
            product_name: request.product_name,
            product_version: request.product_version,
            report_date: Utc::now().date_naive(),
            vendor_name: request.vendor_name,
            vendor_contact: request.vendor_contact,
            wcag_level: request.wcag_level,
            section_508_compliant: request.include_section_508,
            en_301_549_compliant: request.include_en_301_549,
            overall_compliance_score: 0.0,
            total_criteria: 0,
            passed_criteria: 0,
            partially_met_criteria: 0,
            not_met_criteria: 0,
            not_applicable_criteria: 0,
            remarks: None,
            criteria: vec![],
        };
        
        // Auto-populate with WCAG 2.1 AA criteria
        report.criteria = Self::generate_wcag_criteria(report.id, request.wcag_level);
        report.total_criteria = report.criteria.len() as i32;
        report.calculate_compliance_score();
        
        report
    }
    
    /// Generate standard WCAG 2.1 Level A & AA criteria
    fn generate_wcag_criteria(report_id: Uuid, level: WcagConformanceLevel) -> Vec<VpatCriterion> {
        let mut criteria = vec![];
        
        // Perceivable - Text Alternatives
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "1.1.1".to_string(),
            criterion_name: "Non-text Content".to_string(),
            wcag_criterion: Some("All non-text content has text alternatives".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("Images, icons, and media have appropriate alt text".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Perceivable - Time-based Media
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "1.2.1".to_string(),
            criterion_name: "Audio-only and Video-only (Prerecorded)".to_string(),
            wcag_criterion: Some("Alternatives provided for time-based media".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Perceivable - Adaptable
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "1.3.1".to_string(),
            criterion_name: "Info and Relationships".to_string(),
            wcag_criterion: Some("Information, structure, and relationships conveyed through presentation".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("Proper heading hierarchy, form labels, and ARIA attributes used".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Perceivable - Distinguishable
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "1.4.1".to_string(),
            criterion_name: "Use of Color".to_string(),
            wcag_criterion: Some("Color is not the only visual means of conveying information".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "1.4.3".to_string(),
            criterion_name: "Contrast (Minimum)".to_string(),
            wcag_criterion: Some("Text has contrast ratio of at least 4.5:1".to_string()),
            conformance_level: Some(WcagConformanceLevel::AA),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("All text meets minimum contrast requirements".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Operable - Keyboard Accessible
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "2.1.1".to_string(),
            criterion_name: "Keyboard".to_string(),
            wcag_criterion: Some("All functionality available from keyboard".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("Full keyboard navigation supported".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Operable - Enough Time
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "2.2.1".to_string(),
            criterion_name: "Timing Adjustable".to_string(),
            wcag_criterion: Some("Users can turn off, adjust, or extend time limits".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::PartiallySupports,
            user_notes: Some("Most time limits adjustable; some third-party integrations need work".to_string()),
            evidence_url: None,
            remediation_plan: Some("Review and fix timeout behavior in video player integration".to_string()),
            target_remediation_date: Some(Utc::now().date_naive()),
        });
        
        // Operable - Seizures and Physical Reactions
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "2.3.1".to_string(),
            criterion_name: "Three Flashes or Below Threshold".to_string(),
            wcag_criterion: Some("Nothing flashes more than three times per second".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Operable - Navigable
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "2.4.1".to_string(),
            criterion_name: "Bypass Blocks".to_string(),
            wcag_criterion: Some("Mechanism to bypass repeated content".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("Skip to main content links provided".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "2.4.6".to_string(),
            criterion_name: "Headings and Labels".to_string(),
            wcag_criterion: Some("Headings and labels describe topic or purpose".to_string()),
            conformance_level: Some(WcagConformanceLevel::AA),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Understandable - Readable
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "3.1.1".to_string(),
            criterion_name: "Language of Page".to_string(),
            wcag_criterion: Some("Default human language can be programmatically determined".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Understandable - Predictable
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "3.2.1".to_string(),
            criterion_name: "On Focus".to_string(),
            wcag_criterion: Some("No context change on focus".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Understandable - Input Assistance
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "3.3.1".to_string(),
            criterion_name: "Error Identification".to_string(),
            wcag_criterion: Some("Input errors automatically detected and described".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("Form validation provides clear error messages".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // Robust - Compatible
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "4.1.1".to_string(),
            criterion_name: "Parsing".to_string(),
            wcag_criterion: Some("Complete and valid markup".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: None,
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        criteria.push(VpatCriterion {
            id: Uuid::new_v4(),
            report_id,
            criterion_number: "4.1.2".to_string(),
            criterion_name: "Name, Role, Value".to_string(),
            wcag_criterion: Some("UI components have proper name, role, and value".to_string()),
            conformance_level: Some(WcagConformanceLevel::A),
            conformance_status: ConformanceStatus::Supports,
            user_notes: Some("ARIA attributes properly implemented".to_string()),
            evidence_url: None,
            remediation_plan: None,
            target_remediation_date: None,
        });
        
        // If AAA level requested, add additional criteria
        if level == WcagConformanceLevel::AAA {
            criteria.push(VpatCriterion {
                id: Uuid::new_v4(),
                report_id,
                criterion_number: "1.4.6".to_string(),
                criterion_name: "Contrast (Enhanced)".to_string(),
                wcag_criterion: Some("Text has contrast ratio of at least 7:1".to_string()),
                conformance_level: Some(WcagConformanceLevel::AAA),
                conformance_status: ConformanceStatus::PartiallySupports,
                user_notes: Some("Most text meets enhanced contrast; some legacy components need update".to_string()),
                evidence_url: None,
                remediation_plan: Some("Update color scheme in legacy dashboard widgets".to_string()),
                target_remediation_date: Some(Utc::now().date_naive()),
            });
        }
        
        criteria
    }
    
    /// Calculate overall compliance score based on criteria statuses
    pub fn calculate_compliance_score(&mut self) {
        let mut total_weight = 0.0;
        let mut earned_weight = 0.0;
        
        for criterion in &self.criteria {
            let weight = match criterion.conformance_level {
                Some(WcagConformanceLevel::A) => 1.0,
                Some(WcagConformanceLevel::AA) => 1.5,
                Some(WcagConformanceLevel::AAA) => 2.0,
                None => 1.0,
            };
            
            if criterion.conformance_status != ConformanceStatus::NotApplicable {
                total_weight += weight;
                
                let earned = match criterion.conformance_status {
                    ConformanceStatus::Supports => weight,
                    ConformanceStatus::PartiallySupports => weight * 0.5,
                    ConformanceStatus::DoesNotSupport => 0.0,
                    ConformanceStatus::NotApplicable => 0.0,
                };
                
                earned_weight += earned;
            } else {
                self.not_applicable_criteria += 1;
            }
            
            match criterion.conformance_status {
                ConformanceStatus::Supports => self.passed_criteria += 1,
                ConformanceStatus::PartiallySupports => self.partially_met_criteria += 1,
                ConformanceStatus::DoesNotSupport => self.not_met_criteria += 1,
                ConformanceStatus::NotApplicable => {},
            }
        }
        
        self.overall_compliance_score = if total_weight > 0.0 {
            (earned_weight / total_weight) * 100.0
        } else {
            0.0
        };
        
        self.section_508_compliant = self.overall_compliance_score >= 90.0;
        self.en_301_549_compliant = self.overall_compliance_score >= 85.0;
    }
    
    /// Export VPAT report to PDF-ready format
    pub fn export_to_pdf_format(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("VPAT® 2.4 International Edition\n"));
        output.push_str(&format!("Product Name: {}\n", self.product_name));
        output.push_str(&format!("Version: {}\n", self.product_version));
        output.push_str(&format!("Report Date: {}\n", self.report_date));
        output.push_str(&format!("Vendor: {}\n\n", self.vendor_name));
        
        output.push_str("SUMMARY TABLE\n");
        output.push_str(&format!("Overall Compliance Score: {:.1}%\n", self.overall_compliance_score));
        output.push_str(&format!("Section 508 Compliant: {}\n", if self.section_508_compliant { "Yes" } else { "No" }));
        output.push_str(&format!("EN 301 549 Compliant: {}\n\n", if self.en_301_549_compliant { "Yes" } else { "No" }));
        
        output.push_str("DETAILED CRITERIA\n");
        output.push_str(&format!("{:<10} {:<50} {:<25}\n", "Criterion", "Name", "Status"));
        output.push_str(&"-".repeat(85));
        output.push('\n');
        
        for criterion in &self.criteria {
            output.push_str(&format!(
                "{:<10} {:<50} {:<25}\n",
                criterion.criterion_number,
                &criterion.criterion_name[..criterion.criterion_name.len().min(49)],
                criterion.conformance_status.as_str()
            ));
        }
        
        output
    }
    
    /// Identify criteria needing remediation
    pub fn get_remediation_items(&self) -> Vec<&VpatCriterion> {
        self.criteria.iter()
            .filter(|c| matches!(c.conformance_status, ConformanceStatus::DoesNotSupport | ConformanceStatus::PartiallySupports))
            .collect()
    }
}
