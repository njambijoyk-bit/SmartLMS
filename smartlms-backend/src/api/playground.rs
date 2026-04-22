// API Playground - Interactive GraphQL and REST API explorer
// Serves a web-based interface for testing the Developer Platform APIs

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use crate::utils::app_state::AppState;

/// GraphQL Playground HTML
const GRAPHQL_PLAYGROUND_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
  <meta charset=utf-8/>
  <meta name="viewport" content="user-scalable=no, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, minimal-ui">
  <title>SmartLMS GraphQL Playground</title>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
  <link rel="shortcut icon" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
  <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
</head>
<body>
  <div id="root">
    <style>
      body { background-color: rgb(23, 42, 58); font-family: Open Sans, sans-serif; height: 90vh; }
      #root { height: 100%; width: 100%; display: flex; align-items: center; justify-content: center; }
      .loading { font-size: 32px; font-weight: 200; color: rgba(255, 255, 255, .6); margin-left: 28px; }
      img { width: 78px; height: 78px; }
      .title { font-weight: 400; }
    </style>
    <img src='https://cdn.jsdelivr.net/npm/graphql-playground-react/build/logo.png' alt='GraphQL Playground Logo'>
    <div class="loading">Loading <span class="title">GraphQL Playground</span></div>
  </div>
  <script>window.addEventListener('load', function (event) {
      const root = document.getElementById('root');
      root.classList.add('loading');
      GraphQLPlayground.init(root, {
        endpoint: '/graphql',
        settings: {
          'request.credentials': 'include',
          'editor.theme': 'dark',
          'editor.fontSize': 14,
          'editor.fontFamily': "'Source Code Pro', 'Consolas', 'Inconsolata', 'Droid Sans Mono', 'Monaco', monospace",
        },
        tabs: [
          {
            name: 'Developer API',
            endpoint: '/graphql',
            query: `# Welcome to SmartLMS GraphQL API!
# 
# This playground lets you explore and test our GraphQL API.
# Here are some example queries to get you started:

# Get your API keys
query GetApiKeys {
  apiKeys {
    id
    name
    keyPrefix
    permissions
    rateLimit
    isActive
    createdAt
  }
}

# List webhooks
query GetWebhooks {
  webhooks {
    id
    name
    url
    events
    isActive
    createdAt
  }
}

# List integrations
query GetIntegrations {
  integrations {
    id
    name
    integrationType
    isActive
    lastSyncAt
    syncStatus
  }
}

# Get usage statistics
query GetUsageStats {
  usageStats(days: 30) {
    totalRequests
    successfulRequests
    failedRequests
    avgResponseTimeMs
    periodStart
    periodEnd
  }
}

# Create a new webhook
mutation CreateWebhook {
  createWebhook(input: {
    name: "My Webhook"
    url: "https://example.com/webhook"
    events: ["course.created", "user.enrolled"]
  }) {
    webhook {
      id
      name
      url
      events
    }
    secret
  }
}

# Get SDK configuration
query GetSdkConfig {
  sdkConfig {
    apiKey
    baseUrl
    version
    features
  }
}
`,
          },
          {
            name: 'Courses',
            endpoint: '/graphql',
            query: `# Query courses with pagination
query GetCourses {
  courses(first: 10) {
    edges {
      cursor
      node {
        id
        name
        description
        isPublished
        createdAt
      }
    }
    pageInfo {
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }
  }
}

# Get a specific user
query GetUser($userId: ID!) {
  user(id: $userId) {
    id
    email
    name
    role
    institutionId
  }
}
`,
            variables: { userId: "" },
          },
        ],
      });
    });
  </script>
</body>
</html>
"#;

/// REST API Documentation HTML
const REST_API_DOCS_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>SmartLMS REST API Documentation</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif; line-height: 1.6; color: #333; max-width: 1200px; margin: 0 auto; padding: 20px; }
    h1 { color: #2c3e50; margin-bottom: 10px; }
    h2 { color: #34495e; margin: 30px 0 15px; border-bottom: 2px solid #3498db; padding-bottom: 10px; }
    h3 { color: #7f8c8d; margin: 20px 0 10px; }
    .endpoint { background: #f8f9fa; border-left: 4px solid #3498db; padding: 15px; margin: 15px 0; border-radius: 4px; }
    .method { display: inline-block; padding: 4px 8px; border-radius: 3px; font-weight: bold; font-size: 12px; margin-right: 10px; }
    .method.get { background: #61affe; color: white; }
    .method.post { background: #49cc90; color: white; }
    .method.put { background: #fca130; color: white; }
    .method.delete { background: #f93e3e; color: white; }
    .path { font-family: 'Courier New', monospace; font-size: 14px; }
    .description { margin-top: 10px; color: #666; }
    code { background: #f4f4f4; padding: 2px 6px; border-radius: 3px; font-family: 'Courier New', monospace; }
    pre { background: #2d2d2d; color: #f8f8f2; padding: 15px; border-radius: 5px; overflow-x: auto; margin: 15px 0; }
    pre code { background: none; padding: 0; }
    table { width: 100%; border-collapse: collapse; margin: 15px 0; }
    th, td { border: 1px solid #ddd; padding: 10px; text-align: left; }
    th { background: #f8f9fa; font-weight: 600; }
    .note { background: #fff3cd; border-left: 4px solid #ffc107; padding: 15px; margin: 15px 0; }
    .auth { background: #d1ecf1; border-left: 4px solid #17a2b8; padding: 15px; margin: 15px 0; }
  </style>
</head>
<body>
  <h1>🚀 SmartLMS Developer Platform API</h1>
  <p>Welcome to the SmartLMS REST API documentation. Use these endpoints to integrate with SmartLMS.</p>
  
  <div class="auth">
    <strong>🔐 Authentication:</strong> All API requests require authentication. Include your API key in the <code>Authorization</code> header:
    <pre><code>Authorization: Bearer sk_live_your_api_key_here</code></pre>
  </div>

  <h2>API Keys Management</h2>
  
  <div class="endpoint">
    <span class="method post">POST</span>
    <span class="path">/api/v1/developer/api-keys</span>
    <div class="description">Create a new API key</div>
    <pre><code>{
  "name": "My Integration",
  "permissions": ["courses:read", "users:read"],
  "rate_limit": 1000,
  "expires_in_days": 365
}</code></pre>
  </div>

  <div class="endpoint">
    <span class="method get">GET</span>
    <span class="path">/api/v1/developer/api-keys</span>
    <div class="description">List all API keys for current user</div>
  </div>

  <div class="endpoint">
    <span class="method delete">DELETE</span>
    <span class="path">/api/v1/developer/api-keys/:key_id</span>
    <div class="description">Revoke an API key</div>
  </div>

  <h2>Webhooks</h2>

  <div class="endpoint">
    <span class="method post">POST</span>
    <span class="path">/api/v1/developer/webhooks</span>
    <div class="description">Create a new webhook endpoint</div>
    <pre><code>{
  "name": "Course Updates",
  "url": "https://your-app.com/webhook",
  "events": ["course.created", "course.updated", "user.enrolled"]
}</code></pre>
  </div>

  <div class="endpoint">
    <span class="method get">GET</span>
    <span class="path">/api/v1/developer/webhooks</span>
    <div class="description">List all webhooks</div>
  </div>

  <div class="endpoint">
    <span class="method post">POST</span>
    <span class="path">/api/v1/developer/webhooks/:webhook_id/toggle</span>
    <div class="description">Toggle webhook active status</div>
  </div>

  <div class="endpoint">
    <span class="method delete">DELETE</span>
    <span class="path">/api/v1/developer/webhooks/:webhook_id</span>
    <div class="description">Delete a webhook</div>
  </div>

  <h2>Integrations</h2>

  <div class="endpoint">
    <span class="method post">POST</span>
    <span class="path">/api/v1/developer/integrations</span>
    <div class="description">Create a new integration</div>
    <pre><code>{
  "name": "Moodle Sync",
  "integration_type": "moodle",
  "config": {
    "moodle_url": "https://moodle.example.com",
    "token": "your_moodle_token"
  }
}</code></pre>
  </div>

  <div class="endpoint">
    <span class="method get">GET</span>
    <span class="path">/api/v1/developer/integrations</span>
    <div class="description">List all integrations</div>
  </div>

  <div class="endpoint">
    <span class="method delete">DELETE</span>
    <span class="path">/api/v1/developer/integrations/:integration_id</span>
    <div class="description">Delete an integration</div>
  </div>

  <h2>SDK Configuration</h2>

  <div class="endpoint">
    <span class="method get">GET</span>
    <span class="path">/api/v1/developer/sdk/config</span>
    <div class="description">Get SDK configuration for your institution</div>
  </div>

  <div class="endpoint">
    <span class="method get">GET</span>
    <span class="path">/api/v1/developer/rate-limit?api_key=sk_xxx&endpoint=/courses</span>
    <div class="description">Check rate limit status</div>
  </div>

  <h2>Analytics</h2>

  <div class="endpoint">
    <span class="method get">GET</span>
    <span class="path">/api/v1/developer/usage/stats?days=30</span>
    <div class="description">Get API usage statistics</div>
  </div>

  <h2>Event Types</h2>
  <table>
    <tr>
      <th>Event</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>user.created</code></td>
      <td>Triggered when a new user is created</td>
    </tr>
    <tr>
      <td><code>user.enrolled</code></td>
      <td>Triggered when a user enrolls in a course</td>
    </tr>
    <tr>
      <td><code>course.created</code></td>
      <td>Triggered when a new course is created</td>
    </tr>
    <tr>
      <td><code>course.published</code></td>
      <td>Triggered when a course is published</td>
    </tr>
    <tr>
      <td><code>assessment.submitted</code></td>
      <td>Triggered when a student submits an assessment</td>
    </tr>
    <tr>
      <td><code>grade.assigned</code></td>
      <td>Triggered when a grade is assigned</td>
    </tr>
  </table>

  <div class="note">
    <strong>💡 Tip:</strong> For interactive API testing, visit the <a href="/graphql">GraphQL Playground</a>.
  </div>

  <footer style="margin-top: 50px; padding-top: 20px; border-top: 1px solid #eee; color: #666; font-size: 14px;">
    <p>SmartLMS Developer Platform API v1.0 | <a href="/graphql">GraphQL Playground</a></p>
  </footer>
</body>
</html>
"#;

pub async fn graphql_playground() -> Html<&'static str> {
    Html(GRAPHQL_PLAYGROUND_HTML)
}

pub async fn rest_api_docs() -> Html<&'static str> {
    Html(REST_API_DOCS_HTML)
}

pub fn playground_router() -> Router<AppState> {
    Router::new()
        .route("/graphql", get(graphql_playground))
        .route("/docs", get(rest_api_docs))
}
