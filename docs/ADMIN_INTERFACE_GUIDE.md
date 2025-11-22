# EIQ Manager Admin Interface Guide

## Overview

EIQ Manager has a comprehensive admin interface at `/admin` that provides oversight and management capabilities for the entire platform, including desktop app releases, modules, users, and system configuration.

**Access**: `http://localhost:8547/admin` or `https://evolvepreneuriq.app/admin`
**Required Role**: `ROLE_ADMIN`

---

## Admin Dashboard Sections

### 1. **Desktop App Releases** (`/admin/app-releases`)

**Purpose**: Manage desktop app versions, downloads, and GitHub Actions integration

#### Features:
- **Release Management**
  - View all desktop app releases
  - Track download counts per release
  - Mark releases as "Latest" (what users see)
  - Stable vs Beta labeling
  - Multi-platform support (Windows, Linux .deb, Linux AppImage, macOS)

- **GitHub Actions Integration**
  - Configure GitHub Personal Access Token
  - Trigger multi-platform builds from admin panel
  - Webhook integration for auto-populating releases
  - Build status tracking

- **Statistics Dashboard**
  - Total releases count
  - Stable releases count
  - Total downloads (all versions)
  - Latest version indicator

#### How to Use:

**View Releases**:
```
http://localhost:8547/admin/app-releases
```
Shows table with:
- Version number
- Release date
- Status (Stable/Beta)
- Download count
- Available platforms (Win, Deb, AppImage, macOS)
- Last updated date

**Trigger New Build**:
1. Click "Build & Release" button
2. Enter version number (e.g., 1.0.14)
3. Click "Deploy to GitHub"
4. GitHub Actions runs multi-platform build
5. Release auto-populates via webhook

**Upload Manual Release**:
1. Click "Upload Files" button
2. Enter version details
3. Upload installer files or paste download URLs
4. Mark as Latest/Stable
5. Save

**Configure GitHub Integration**:
1. Expand "GitHub Actions Integration" section
2. Create GitHub Personal Access Token:
   - Go to https://github.com/settings/tokens/new
   - Required scopes: `repo`, `workflow`
3. Paste token and save
4. Webhook secret auto-generated

#### Current Status:
- ✅ Build triggering working
- ✅ Webhook integration configured
- ✅ Download tracking active
- ✅ Multi-platform builds (Windows + Linux)
- ⚠️ macOS builds disabled (cost savings)

---

### 2. **Build & Release Trigger** (`/admin/build-release/trigger`)

**Purpose**: Trigger GitHub Actions workflow to build desktop app for all platforms

#### Process:
1. Enter version number (e.g., 1.0.15)
2. System triggers GitHub Actions workflow
3. Workflow builds:
   - Windows (.msi, .exe)
   - Linux (.deb, .AppImage)
   - ~~macOS (.dmg, .app)~~ (disabled)
4. Builds take ~5-7 minutes
5. GitHub publishes release
6. Webhook calls back to EIQ Manager
7. Release appears in `/admin/app-releases` automatically

#### Workflow Details:
- **Repository**: `https://github.com/evolvesystems/evolve-desktop`
- **Workflow File**: `.github/workflows/build-release.yml`
- **Trigger URL**: `https://api.github.com/repos/evolvesystems/evolve-desktop/actions/workflows/build-release.yml/dispatches`
- **Webhook**: `https://evolvepreneuriq.app/webhook/github/release`

---

### 3. **Developer Dashboard** (`/admin/developer`)

**Purpose**: Developer tools and system information

Likely includes:
- API documentation links
- System health checks
- Database status
- Cache management
- Debug tools

---

### 4. **Module Management** (`/admin/modules`)

**Purpose**: Enable/disable modules per tenant

#### Available Modules:
Based on the exploration, these modules can be managed:
- Email Manager
- CRM Marketing
- Chat/Messaging
- Books (EvolveWriter)
- Video Meetings
- Knowledge Base
- Ticketing
- E-commerce
- Page Builder
- RSS Widgets
- Social Sharing
- Mobile App Builder
- Scheduling
- And more...

---

### 5. **User Management** (`/admin/users`)

**Purpose**: Manage users across all tenants

#### Features:
- View all users (`show_users.html.twig`)
- Create new user (`create_user.html.twig`)
- Edit user details (`edit_user.html.twig`)
- Assign roles
- Manage permissions
- View user activity

---

### 6. **Tenant Management** (`/admin/tenants`)

**Purpose**: Manage multi-tenant accounts

#### Features:
- Create new tenants
- Edit tenant settings
- View tenant usage/stats
- Tenant impersonation (`/admin/tenant_impersonation`)
- Billing management

---

### 7. **System Settings** (`/admin/settings`)

Configuration sections:

#### A. Mailer Settings (`/admin/mailer_settings`)
- SMTP configuration
- Email sending settings
- Mail queue management

#### B. App Settings (`/admin/app_settings`)
- System-wide configurations
- API settings
- Feature flags

#### C. Analytics Settings (`/admin/matomo_settings`)
- Matomo integration
- Analytics tracking
- Privacy settings

#### D. SMS Settings (`/admin/sms_settings`)
- SMS gateway configuration
- SMS notification settings

---

### 8. **Email Hosting** (`/admin/email_hosting`)

**Purpose**: Manage email hosting accounts and configuration

Features:
- Email account management
- Server configuration
- IMAP/SMTP settings
- Email quotas

---

### 9. **Knowledge Base Admin** (`/admin/knowledgebase`)

**Purpose**: Manage knowledge base articles and categories

Features:
- Article management
- Category organization
- Publishing workflow
- Search configuration

---

### 10. **Chatbot Management** (`/admin/chatbot`)

**Purpose**: Manage AI chatbot profiles and training

Features:
- Chatbot profile configuration
- Training data management
- RAG (Retrieval Augmented Generation) setup
- Analytics

---

### 11. **Integration Management** (`/admin/integration`)

**Purpose**: Configure 3rd-party integrations

Available integrations:
- Mautic (marketing automation)
- Zoom (video meetings)
- Daily.co (video meetings)
- LinkedIn (social, events)
- Payment gateways

---

### 12. **Template Management** (`/admin/templates`)

**Purpose**: Manage system templates

Template types:
- Email templates
- Invoice templates
- Ticket response templates
- Campaign templates

---

### 13. **Workflow Management** (`/admin/workflow`)

**Purpose**: Configure automation workflows

Features:
- Visual workflow builder
- Trigger configuration
- Action setup
- Conditional logic

---

### 14. **Training Centre** (`/admin/training_centre`)

**Purpose**: AI training content management

Features:
- Upload training documents
- Manage knowledge sources
- Configure RAG system
- Monitor AI performance

---

### 15. **Logs & Monitoring** (`/admin/logs`)

**Purpose**: System logging and monitoring

Features:
- Application logs
- Error tracking
- Activity monitoring
- Performance metrics

---

### 16. **Notifications** (`/admin/notifications`)

**Purpose**: System notification configuration

Features:
- Notification templates
- Delivery settings
- User preferences
- Notification history

---

### 17. **Payment Management** (`/admin/payment`)

**Purpose**: Payment processing configuration

Features:
- Payment gateway setup
- Transaction history
- Refund management
- Invoice generation

---

### 18. **Scheduling Admin** (`/admin/scheduling`)

**Purpose**: Appointment and booking management

Features:
- Calendar configuration
- Booking settings
- Availability management
- Appointment types

---

### 19. **Ticket Settings** (`/admin/ticket_settings`)

**Purpose**: Support ticket system configuration

Features:
- Department management (`/admin/ticket_department`)
- Template management (`/admin/ticket_template`)
- Priority/status configuration
- SLA settings

---

### 20. **CRM Admin** (`/admin/crm`)

**Purpose**: CRM module configuration

Features:
- Contact field customization
- Pipeline stages
- Lead scoring rules
- Custom fields

---

### 21. **LinkedIn Integration** (`/admin/linkedin`)

**Purpose**: LinkedIn integration management

Features:
- OAuth configuration
- Auto-posting settings
- Event synchronization
- Analytics

---

### 22. **Proofpoint Integration** (`/admin/proofpoint`)

**Purpose**: Email security integration

Features:
- Proofpoint API configuration
- Security policies
- Threat monitoring
- Quarantine management

---

### 23. **Database Management** (`/admin/database`)

**Purpose**: Database administration tools

Features likely include:
- Schema management
- Backup/restore
- Query tools
- Performance monitoring

---

### 24. **Books Admin** (`/admin/books`)

**Purpose**: EvolveWriter book platform administration

Features:
- Book moderation
- User access management
- Export settings
- Collaboration tools

---

### 25. **Conversations** (`/admin/conversations`)

**Purpose**: Chat/messaging administration

Features:
- Channel management
- Message moderation
- User access control
- Archive management

---

### 26. **Chat Admin** (`/admin/chat`)

**Purpose**: Real-time chat system management

Features:
- Channel creation
- User permissions
- File upload settings
- Retention policies

---

### 27. **E-commerce Admin** (`/admin/ecommerce`)

**Purpose**: Online store management

Features:
- Product catalog
- Order management
- Inventory tracking
- Shipping settings
- Payment configuration

---

### 28. **Invoice Management** (`/admin/invoice`)

**Purpose**: Invoice generation and tracking

Features:
- Invoice creation
- Template customization (`/admin/invoice_template`)
- Payment tracking
- PDF generation

---

### 29. **Navigation Admin** (`/admin/navigation`)

**Purpose**: Site navigation management

Features:
- Menu builder
- Navigation structure
- Link management
- Permission-based menus

---

### 30. **Role Management** (`/admin/roles`)

**Purpose**: User role and permission management

Features:
- Create/edit roles
- Assign permissions
- User-role mapping
- Access control

---

### 31. **Email History** (`/admin/email_history`)

**Purpose**: Email delivery tracking

Features:
- Sent email log
- Delivery status
- Bounce tracking
- Email analytics

---

### 32. **File Management** (`/admin/files`)

**Purpose**: File storage administration

Features:
- File browser
- Upload management
- Storage quotas
- CDN settings

---

### 33. **Billing Admin** (`/admin/billing`)

**Purpose**: Billing and subscription management

Features:
- Subscription plans
- Payment history
- Invoice generation
- Usage tracking

---

### 34. **Domain Management** (`/admin/domains`)

**Purpose**: Domain name administration

Features:
- Domain registration
- DNS management
- SSL certificates
- Domain transfers

---

### 35. **Report Management** (`/admin/report`)

**Purpose**: Analytics and reporting

Features:
- Custom report builder
- Scheduled reports
- Export functionality
- Dashboard creation

---

### 36. **Cron Jobs** (`/admin/cron`)

**Purpose**: Scheduled task management

Features:
- Job scheduling
- Execution history
- Error tracking
- Manual triggers

---

### 37. **Typesense Admin** (`/admin/typesense`)

**Purpose**: Search engine configuration

Features:
- Index management
- Search configuration
- Performance tuning
- Query analytics

---

### 38. **System Healing** (`/admin/system_healing`)

**Purpose**: Automatic system maintenance

Features:
- Health checks
- Auto-repair tools
- Consistency checks
- Database optimization

---

### 39. **Template Showcase** (`/admin/template_showcase`)

**Purpose**: Template gallery and demos

Features:
- Template preview
- Component showcase
- Design system documentation

---

### 40. **Agreements & Contracts** (`/admin/agreements_contracts`)

**Purpose**: Legal document management

Features:
- Contract templates
- Agreement tracking
- Signature management
- Version control

---

## Accessing the Admin Interface

### Login as Admin:
```bash
# Visit login page
http://localhost:8547/login

# Login with admin credentials
Email: admin@example.com
Password: [your admin password]

# After login, visit:
http://localhost:8547/admin
```

### If redirected to Developer Dashboard:
The main `/admin` route redirects to `/admin/developer` by default. Use the side navigation to access other sections.

---

## Admin Interface Architecture

### Template Structure:
```
templates/admin/
├── app_releases/           # Desktop app release management
│   ├── index.html.twig     # Release list
│   ├── new.html.twig       # Create release
│   └── edit.html.twig      # Edit release
├── build_release/          # GitHub Actions trigger
│   └── form.html.twig      # Build trigger form
├── developer/              # Developer dashboard
├── modules/                # Module management
├── users/                  # User management
│   ├── show_users.html.twig
│   ├── create_user.html.twig
│   └── edit_user.html.twig
└── [50+ other sections]
```

### Controller Structure:
```php
src/Controller/
├── AdminController.php                  # Main admin controller
├── Admin/
│   ├── AdminDashboardController.php     # Dashboard
│   ├── KnowledgebaseAdminController.php
│   ├── ProofpointAdminController.php
│   └── [many others]
└── [module-specific admin controllers]
```

---

## Key Features for Desktop App Management

### 1. Release Version Tracking
- See all desktop app versions in one place
- Track which version is "Latest"
- Monitor download statistics
- Platform availability (Win, Linux, macOS)

### 2. Build Automation
- One-click GitHub Actions builds
- Multi-platform compilation
- Automatic release creation
- Webhook integration for updates

### 3. Download Management
- Centralized download URLs
- CDN integration
- Download count tracking
- Platform-specific installers

### 4. API Monitoring
- View active API endpoints
- Check connection status
- Troubleshoot desktop connectivity
- CORS configuration

### 5. User Support
- View all desktop app users
- Access user accounts for troubleshooting
- Monitor usage statistics
- Handle support tickets

---

## Recommended Admin Workflow

### For Desktop App Releases:

1. **Development Complete** → Test locally
2. **Commit & Push** → Push to GitHub
3. **Admin Panel** → Visit `/admin/build-release/trigger`
4. **Enter Version** → e.g., 1.0.15
5. **Click Deploy** → Triggers GitHub Actions
6. **Wait 5-7 min** → Builds complete
7. **Check Releases** → Visit `/admin/app-releases`
8. **Verify Download** → Test installer downloads
9. **Mark Latest** → If stable, mark as latest release
10. **Announce** → Notify users of new version

### For Feature Visibility:

1. **Check Modules** → Visit `/admin/modules`
2. **Review Features** → See what's enabled
3. **Cross-reference** → Compare with `FEATURE_PARITY_MATRIX.md`
4. **Plan Desktop** → Prioritize features to replicate
5. **Update Tracking** → Update feature parity docs

---

## Tips & Best Practices

### Desktop App Releases:
- Always mark stable releases clearly
- Keep at least 2-3 old versions available
- Test installers before marking as "Latest"
- Document release notes in changelog

### Module Management:
- Audit enabled modules quarterly
- Disable unused modules to improve performance
- Document module dependencies
- Test module interactions

### User Management:
- Use role-based access control
- Regular permission audits
- Monitor failed login attempts
- Keep user count synced with billing

### System Maintenance:
- Check logs weekly
- Review cron job execution
- Monitor database size
- Clean up old files/data

---

## Troubleshooting

### Can't Access Admin:
```bash
# Check if user has ROLE_ADMIN
symfony console app:user:check-role admin@example.com

# Grant admin role
symfony console app:user:promote admin@example.com ROLE_ADMIN
```

### Builds Not Triggering:
1. Check GitHub token is configured
2. Verify token has `repo` and `workflow` scopes
3. Check GitHub Actions status
4. Review error logs in `/admin/logs`

### Webhook Not Working:
1. Verify webhook secret matches
2. Check webhook URL is accessible
3. Test webhook with curl
4. Review webhook logs in GitHub

---

## Admin API Endpoints

For programmatic access:

```bash
# List releases
GET /api/v1/admin/releases

# Create release
POST /api/v1/admin/releases

# Trigger build
POST /admin/build-release/trigger

# Get system stats
GET /api/v1/admin/stats
```

---

## Security Considerations

### Admin Access:
- ✅ Requires `ROLE_ADMIN`
- ✅ Multi-factor authentication supported
- ✅ Activity logging enabled
- ✅ Session timeout configured

### API Security:
- ✅ JWT token authentication
- ✅ CORS properly configured
- ✅ Rate limiting (planned)
- ✅ Input validation

### Data Protection:
- ✅ Passwords hashed
- ✅ Tokens encrypted
- ✅ Audit logs maintained
- ✅ Backup strategy in place

---

## Summary

The EIQ Manager admin interface provides:

- **40+ management sections** covering all platform aspects
- **Desktop app release management** with GitHub Actions integration
- **Module configuration** to enable/disable features
- **User & tenant management** for multi-tenant SaaS
- **System monitoring** and health checks
- **Integration management** for 3rd-party services
- **Complete oversight** of platform operations

**Access**: `http://localhost:8547/admin`
**Key Section**: `/admin/app-releases` for desktop app management

---

**Last Updated**: 2025-01-22
**Document Version**: 1.0.0
**Maintained By**: Platform Team
