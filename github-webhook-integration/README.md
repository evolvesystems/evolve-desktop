# GitHub Webhook Integration for EvolveApp Desktop Releases

This integration automatically updates your admin panel at `http://localhost:8547/admin/app-releases` with real-time updates when GitHub releases are published.

## How It Works

1. When you push a new tag (e.g., `v1.0.6`), GitHub Actions builds the desktop app for all platforms
2. When the build completes, you manually create a GitHub Release
3. GitHub sends a webhook to your server
4. The webhook controller automatically creates/updates the release in your admin panel
5. Download links point directly to GitHub's CDN

## Setup Instructions

### 1. Install the Webhook Controller

```bash
# Copy the controller to your EIQ Manager project
sudo cp GitHubWebhookController.php /home/john/sources/eiq-manager/src/Controller/Webhook/

# Fix permissions
sudo chown www-data:www-data /home/john/sources/eiq-manager/src/Controller/Webhook/GitHubWebhookController.php
```

### 2. Add Webhook Secret to Environment

Edit `/home/john/sources/eiq-manager/.env`:

```bash
# Add this line with a secure random string
GITHUB_WEBHOOK_SECRET=your_secure_random_secret_here
```

Generate a secure secret:
```bash
openssl rand -hex 32
```

### 3. Configure GitHub Webhook

1. Go to https://github.com/evolvesystems/evolve-desktop/settings/hooks
2. Click **Add webhook**
3. Configure:
   - **Payload URL**: `http://your-server.com:8547/webhook/github/release`
   - **Content type**: `application/json`
   - **Secret**: (paste the same value from GITHUB_WEBHOOK_SECRET)
   - **Which events**: Select "Let me select individual events"
     - ✅ Releases
     - ✅ Workflow runs (optional, for monitoring)
   - **Active**: ✅ Checked

4. Click **Add webhook**

### 4. Test the Integration

#### Method 1: Manual Test

```bash
# Test the endpoint is working
curl -X POST http://localhost:8547/webhook/github/release \
  -H "Content-Type: application/json" \
  -d '{"action": "published", "release": {"tag_name": "v1.0.6", "body": "Test release", "published_at": "2025-01-15T00:00:00Z", "draft": false, "prerelease": false, "assets": []}}'
```

#### Method 2: Create a GitHub Release

1. Go to https://github.com/evolvesystems/evolve-desktop/releases
2. Click **Draft a new release**
3. Choose the `v1.0.6` tag
4. Fill in release notes
5. Click **Publish release**
6. Check your admin panel - the release should appear automatically!

## Webhook Payload Example

When GitHub publishes a release, it sends a payload like this:

```json
{
  "action": "published",
  "release": {
    "tag_name": "v1.0.6",
    "name": "EvolveApp Desktop 1.0.6",
    "body": "## What's New\\n- Feature 1\\n- Bug fix 2",
    "draft": false,
    "prerelease": false,
    "published_at": "2025-01-15T12:00:00Z",
    "assets": [
      {
        "name": "EvolveApp_1.0.6_x64_en-US.msi",
        "browser_download_url": "https://github.com/evolvesystems/evolve-desktop/releases/download/v1.0.6/EvolveApp_1.0.6_x64_en-US.msi"
      },
      {
        "name": "evolveapp_1.0.6_amd64.deb",
        "browser_download_url": "https://github.com/evolvesystems/evolve-desktop/releases/download/v1.0.6/evolveapp_1.0.6_amd64.deb"
      },
      {
        "name": "EvolveApp_1.0.6_amd64.AppImage",
        "browser_download_url": "https://github.com/evolvesystems/evolve-desktop/releases/download/v1.0.6/EvolveApp_1.0.6_amd64.AppImage"
      },
      {
        "name": "EvolveApp_1.0.6_x64.dmg",
        "browser_download_url": "https://github.com/evolvesystems/evolve-desktop/releases/download/v1.0.6/EvolveApp_1.0.6_x64.dmg"
      }
    ]
  }
}
```

## Platform Detection

The webhook automatically detects platform installers based on file extensions:

| Platform | File Extensions | Example |
|----------|----------------|---------|
| Windows | `.msi`, `setup.exe` | `EvolveApp_1.0.6_x64.msi` |
| Linux DEB | `.deb` | `evolveapp_1.0.6_amd64.deb` |
| Linux AppImage | `.appimage` | `EvolveApp_1.0.6_amd64.AppImage` |
| macOS | `.dmg`, `.pkg` | `EvolveApp_1.0.6_x64.dmg` |

## Troubleshooting

### Webhook not receiving events

1. Check GitHub webhook delivery logs: https://github.com/evolvesystems/evolve-desktop/settings/hooks
2. Verify your server is publicly accessible on port 8547
3. Check Symfony logs: `tail -f /home/john/sources/eiq-manager/var/log/dev.log`

### Signature verification failing

1. Ensure `GITHUB_WEBHOOK_SECRET` matches the secret in GitHub webhook settings
2. Check that the header `X-Hub-Signature-256` is present

### Release not appearing in admin panel

1. Check the response from GitHub: Look at the webhook delivery in GitHub settings
2. Verify the `AppRelease` entity exists and has all required methods
3. Check database permissions: `docker-compose exec postgres psql -U postgres -d eiq_manager`

## Security Notes

1. **Always use a strong webhook secret** - This prevents unauthorized requests
2. **Use HTTPS in production** - Never expose webhooks over HTTP in production
3. **Validate signatures** - The controller verifies GitHub's signature on every request
4. **Rate limiting** - Consider adding rate limiting to the webhook endpoint

## Benefits of This Integration

✅ **Automatic** - No manual data entry required
✅ **Real-time** - Releases appear immediately after publishing
✅ **CDN-powered** - Uses GitHub's fast, global CDN for downloads
✅ **Version controlled** - Release assets are tied to git tags
✅ **Audit trail** - All webhook events are logged
