# EvolveApp Deployment Options

## 🎯 Three Deployment Models

You can deploy EvolveApp in three different ways depending on your business model and customer needs.

---

## Option 1: SaaS / Multi-Tenant (Recommended for Most) ⭐

### Architecture
```
Desktop App (Users download) → YOUR Hosted API → YOUR Database
```

### How It Works

**You Host:**
- eiq-manager API at `https://api.evolveapp.com`
- PostgreSQL database (DigitalOcean, AWS RDS, etc.)
- Handle scaling, backups, security

**Users:**
- Download desktop app (or use web version)
- Create account on YOUR platform
- Their data stored in YOUR database (multi-tenant)
- Pay monthly subscription

### Setup

**1. Deploy eiq-manager to production:**
```bash
# Use Docker, Kubernetes, or traditional hosting
# Set production database credentials
DATABASE_URL=postgresql://user:pass@db.host:5432/prod_db
```

**2. Configure desktop app:**
```env
# .env.production
VITE_API_URL=https://api.evolveapp.com
VITE_ALLOW_CUSTOM_API=false  # Users can't change URL
```

**3. Build and distribute:**
```bash
npm run tauri:build
# Distribute via:
# - Website download
# - App stores (Mac App Store, Microsoft Store)
# - Auto-updater
```

### Database Structure (Multi-Tenant)

```sql
-- Each user/organization has their own data partition

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) UNIQUE,
  organization_id INTEGER,  -- Multi-tenant key
  ...
);

CREATE TABLE emails (
  id SERIAL PRIMARY KEY,
  user_id INTEGER,
  organization_id INTEGER,  -- Isolate data
  ...
);

-- Queries always filter by organization_id
SELECT * FROM emails WHERE organization_id = ?
```

### Pros
- ✅ Easy for users - just install and login
- ✅ Recurring revenue (subscriptions)
- ✅ You control updates and features
- ✅ Centralized monitoring and support
- ✅ Better user experience

### Cons
- ❌ Hosting costs (scale with users)
- ❌ Data privacy concerns
- ❌ Need robust infrastructure

### Examples
- Slack, Trello, Notion, Asana

### Revenue Model
- Free tier: 5GB storage, 1 user
- Pro: $10/month - 100GB, 10 users
- Enterprise: $50/month - unlimited, custom features

---

## Option 2: Self-Hosted / On-Premise (Enterprise)

### Architecture
```
Desktop App (Customer downloads) → CUSTOMER'S Server → CUSTOMER'S Database
```

### How It Works

**You Provide:**
- Desktop app installer
- eiq-manager source code or Docker image
- Documentation and setup scripts

**Customer Hosts:**
- Their own eiq-manager instance
- Their own PostgreSQL database
- On their infrastructure (AWS, on-premise, etc.)

**Users:**
- Download desktop app
- Point to company's server: `https://company.internal/api`
- Data stays on company infrastructure

### Setup

**1. Package eiq-manager:**
```bash
# Create Docker image
docker build -t evolveapp/eiq-manager:latest .
docker push evolveapp/eiq-manager:latest

# Or provide tar.gz with source
tar -czf eiq-manager-v1.0.0.tar.gz eiq-manager/
```

**2. Desktop app supports custom URL:**
```env
# .env.production
VITE_ALLOW_CUSTOM_API=true  # Users can configure URL
```

**3. Customer setup instructions:**
```bash
# Customer runs on their server:
docker run -d \
  -p 8547:80 \
  -e DATABASE_URL=postgresql://... \
  evolveapp/eiq-manager:latest

# Desktop app configured to:
https://their-company.com/evolveapp/api
```

### Installation Script (Customer)

```bash
#!/bin/bash
# install-eiq-manager.sh

echo "EIQ Manager Installation"
echo "======================="

# 1. Install Docker
apt-get install -y docker docker-compose

# 2. Create directory
mkdir -p /opt/eiq-manager
cd /opt/eiq-manager

# 3. Download compose file
wget https://evolveapp.com/downloads/docker-compose.yml

# 4. Configure database
read -p "Database URL: " DB_URL
echo "DATABASE_URL=$DB_URL" > .env

# 5. Start services
docker-compose up -d

echo "Installation complete!"
echo "API available at: http://localhost:8547"
```

### Pros
- ✅ Data stays on customer servers (compliance)
- ✅ Full control for customers
- ✅ No hosting costs for you
- ✅ Can charge license fees

### Cons
- ❌ Harder to support
- ❌ Users need technical expertise
- ❌ Update distribution challenges
- ❌ Each customer has different setup

### Examples
- GitLab, Nextcloud, Mattermost

### Revenue Model
- One-time license: $5,000
- Annual support: $1,000/year
- Enterprise features: $10,000+

---

## Option 3: Hybrid / Configurable (Maximum Flexibility) 🎯

### Architecture
```
Desktop App → User Chooses:
  ├─ Your Hosted API (SaaS)
  ├─ Customer's Server (Self-hosted)
  └─ Local Development (localhost)
```

### How It Works

**On First Launch:**
```
┌─────────────────────────────────────┐
│  Welcome to EvolveApp               │
│                                     │
│  Choose your setup:                 │
│  ○ EvolveApp Cloud (Hosted by us)  │
│  ○ Self-hosted server               │
│  ○ Local development                │
│                                     │
│  Custom URL: ________________       │
│                                     │
│  [Test Connection] [Continue]       │
└─────────────────────────────────────┘
```

**Options:**
1. **Cloud** → `https://api.evolveapp.com`
2. **Self-hosted** → Customer enters their URL
3. **Local** → `http://localhost:8547`

### Implementation

See `src/views/ServerConfig.vue` for the configuration screen.

**User's choice saved:**
```typescript
// localStorage stores API URL
localStorage.setItem('api_url', 'https://api.evolveapp.com')

// Axios uses it
axios.defaults.baseURL = localStorage.getItem('api_url')
```

### Pros
- ✅ Supports all customer types
- ✅ Maximum flexibility
- ✅ Can offer both hosted and self-hosted
- ✅ Good for testing and development

### Cons
- ❌ More complex to support
- ❌ Need clear documentation
- ❌ User confusion about options

### Examples
- VS Code (can connect to any server)
- Postman (API client)

---

## 📊 Comparison Table

| Feature | SaaS | Self-Hosted | Hybrid |
|---------|------|-------------|--------|
| **Easy for users** | ✅ Very easy | ❌ Technical | ⚠️ Moderate |
| **Your hosting costs** | ❌ High | ✅ None | ⚠️ Partial |
| **Data privacy** | ⚠️ Your server | ✅ Customer server | ⚠️ Varies |
| **Support complexity** | ✅ Easy | ❌ Hard | ❌ Hard |
| **Revenue potential** | ✅ Recurring | ⚠️ One-time | ✅ Both |
| **Update distribution** | ✅ Immediate | ❌ Manual | ⚠️ Varies |
| **Compliance** | ⚠️ Depends | ✅ Customer control | ⚠️ Varies |

---

## 💡 Recommendation

### For Most Businesses: **Start with SaaS (Option 1)**

**Why:**
- Faster time to market
- Easier to support
- Better user experience
- Predictable revenue

**Then add:** Self-hosted option for enterprise customers

### Implementation Path

**Phase 1:** SaaS only
```env
VITE_API_URL=https://api.evolveapp.com
VITE_ALLOW_CUSTOM_API=false
```

**Phase 2:** Add enterprise self-hosted
- Provide Docker image
- Documentation for IT teams
- Premium support package

**Phase 3:** Hybrid mode
- Add server configuration screen
- Support both deployment models
- Charge accordingly

---

## 🚀 Getting Started

### Deploy Your SaaS Version

**1. Set up production server:**
```bash
# Get VPS (DigitalOcean, AWS, etc.)
# Point domain: api.evolveapp.com

# Deploy eiq-manager
docker-compose up -d
```

**2. Update desktop app:**
```bash
# .env.production
VITE_API_URL=https://api.evolveapp.com

# Build
npm run tauri:build
```

**3. Distribute:**
- Upload to website
- Create installers for Windows/Mac/Linux
- Set up auto-updater

### Enable Self-Hosted Mode

**1. Create deployment package:**
```bash
# Create Docker image
docker build -t your-org/eiq-manager:v1.0 .
docker push your-org/eiq-manager:v1.0
```

**2. Desktop app supports custom URL:**
```bash
# Enable configuration screen
# See ServerConfig.vue
```

**3. Provide documentation:**
- Installation guide
- Firewall configuration
- SSL certificate setup
- Backup procedures

---

## 🔒 Security Considerations

### SaaS Model
- ✅ SSL/TLS required
- ✅ Regular backups
- ✅ Security updates
- ✅ Monitoring
- ✅ GDPR compliance

### Self-Hosted Model
- Customer responsible for:
  - Server security
  - Database backups
  - SSL certificates
  - Updates
  - Compliance

---

## 📋 Checklist: Deploying SaaS

- [ ] Register domain (api.evolveapp.com)
- [ ] Set up VPS/cloud hosting
- [ ] Configure PostgreSQL database
- [ ] Deploy eiq-manager with Docker
- [ ] Set up SSL certificate (Let's Encrypt)
- [ ] Configure CORS for desktop app
- [ ] Set up monitoring (Sentry, etc.)
- [ ] Configure backups
- [ ] Build desktop app with production API URL
- [ ] Create installers (AppImage, DMG, EXE)
- [ ] Set up distribution (website, app stores)
- [ ] Configure auto-updater
- [ ] Create pricing page
- [ ] Set up payment processing (Stripe)

---

## Questions?

**Which model should you use?**
- Small business / startup → SaaS
- Enterprise customers → Self-hosted
- Maximum flexibility → Hybrid

**Current Configuration:**
- Development: localhost:8547
- Can be changed to any model above
