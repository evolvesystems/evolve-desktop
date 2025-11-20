<template>
  <div class="setup-wizard">
    <div class="wizard-container">
      <div class="wizard-header">
        <h1>Welcome to EvolveApp</h1>
        <p class="subtitle">Let's get you connected to your server</p>
        <p class="version">Version {{ appVersion }}</p>
      </div>

      <!-- Step 1: Auto-Discovery -->
      <div v-if="currentStep === 'discovery'" class="wizard-step">
        <div class="icon-container">
          <svg v-if="isScanning" class="scan-icon rotating" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="10" stroke-width="2"/>
            <path d="M12 6v6l4 2" stroke-width="2"/>
          </svg>
          <svg v-else class="server-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <rect x="2" y="2" width="20" height="8" rx="2" ry="2" stroke-width="2"/>
            <rect x="2" y="14" width="20" height="8" rx="2" ry="2" stroke-width="2"/>
            <line x1="6" y1="6" x2="6.01" y2="6" stroke-width="2"/>
            <line x1="6" y1="18" x2="6.01" y2="18" stroke-width="2"/>
          </svg>
        </div>

        <h2>{{ isScanning ? 'Searching for servers...' : 'Found Servers' }}</h2>

        <div v-if="isScanning" class="scanning-status">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: scanProgress + '%' }"></div>
          </div>
          <p>Checking {{ currentScanUrl }}...</p>
        </div>

        <div v-else-if="discoveredServers.length > 0" class="server-list">
          <button
            v-for="server in discoveredServers"
            :key="server.url"
            @click="selectServer(server)"
            class="server-item"
            :class="{ selected: selectedServer?.url === server.url }"
          >
            <div class="server-info">
              <h3>{{ server.name }}</h3>
              <p class="server-url">{{ server.url }}</p>
              <p class="server-status">
                <span class="status-dot" :class="server.status"></span>
                {{ server.version ? `Version ${server.version}` : 'EvolveApp Server' }}
              </p>
            </div>
            <svg v-if="selectedServer?.url === server.url" class="check-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <polyline points="20 6 9 17 4 12" stroke-width="2"/>
            </svg>
          </button>
        </div>

        <div v-else class="no-servers">
          <p>No servers found automatically</p>
          <p class="hint">You can enter a server URL manually below</p>
        </div>

        <button @click="currentStep = 'manual'" class="btn-secondary">
          Enter URL Manually
        </button>

        <button
          v-if="!isScanning && discoveredServers.length > 0"
          @click="testAndConnect"
          :disabled="!selectedServer || isConnecting"
          class="btn-primary"
        >
          {{ isConnecting ? 'Connecting...' : 'Connect' }}
        </button>
      </div>

      <!-- Step 2: Manual Entry -->
      <div v-if="currentStep === 'manual'" class="wizard-step">
        <h2>Enter Server URL</h2>

        <div class="form-group">
          <label for="server-url">Server URL</label>
          <input
            id="server-url"
            v-model="manualUrl"
            type="url"
            placeholder="http://localhost:8547"
            @input="urlError = ''"
          />
          <p v-if="urlError" class="error-message">{{ urlError }}</p>
          <p class="hint">Enter the full URL including http:// or https://</p>
        </div>

        <div class="preset-servers">
          <p class="label">Common Configurations:</p>
          <button
            v-for="preset in presetServers"
            :key="preset.url"
            @click="manualUrl = preset.url"
            class="preset-btn"
          >
            {{ preset.name }}
          </button>
        </div>

        <div class="wizard-actions">
          <button @click="currentStep = 'discovery'" class="btn-secondary">
            Back
          </button>
          <button
            @click="testManualUrl"
            :disabled="!manualUrl || isConnecting"
            class="btn-primary"
          >
            {{ isConnecting ? 'Testing...' : 'Test Connection' }}
          </button>
        </div>
      </div>

      <!-- Step 3: Success -->
      <div v-if="currentStep === 'success'" class="wizard-step success">
        <div class="success-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="10" stroke-width="2"/>
            <polyline points="8 12 11 15 16 10" stroke-width="2"/>
          </svg>
        </div>

        <h2>Connected Successfully!</h2>
        <p>You're all set to use EvolveApp</p>

        <div class="connection-details">
          <p><strong>Server:</strong> {{ connectedServerName }}</p>
          <p><strong>URL:</strong> {{ finalServerUrl }}</p>
        </div>

        <button @click="finishSetup" class="btn-primary">
          Get Started
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'

// Check if running in Tauri or browser
const isTauri = typeof window !== 'undefined' && '__TAURI__' in window

// Dynamically import Tauri API only if in Tauri environment
let invoke: any = null
if (isTauri) {
  import('@tauri-apps/api/core').then(module => {
    invoke = module.invoke
  }).catch(() => {
    console.log('[SetupWizard] Running in browser mode (Tauri API not available)')
  })
}

interface Server {
  url: string
  name: string
  status: 'online' | 'offline'
  version?: string
  responseTime?: number
}

const currentStep = ref<'discovery' | 'manual' | 'success'>('discovery')
const isScanning = ref(true)
const scanProgress = ref(0)
const currentScanUrl = ref('')
const discoveredServers = ref<Server[]>([])
const selectedServer = ref<Server | null>(null)
const isConnecting = ref(false)

const manualUrl = ref('')
const urlError = ref('')

const connectedServerName = ref('')
const finalServerUrl = ref('')

// Preset server configurations
const productionUrl = import.meta.env.VITE_PRODUCTION_URL || 'https://evolvepreneuriq.app'
const localUrl = import.meta.env.VITE_API_URL || 'http://localhost:8547'

const presetServers = [
  { name: 'Production Server', url: productionUrl },
  { name: 'Local Development', url: localUrl },
  { name: 'Local Development (Alt)', url: 'http://localhost:8000' },
  { name: 'Local Network', url: 'http://192.168.1.100:8547' },
]

// Build smart scan targets based on current host
function buildScanTargets(): string[] {
  const currentHost = window.location.hostname
  const targets: string[] = [productionUrl, localUrl]

  // Add current host with port 8547 if not localhost
  if (currentHost !== 'localhost' && currentHost !== '127.0.0.1') {
    const protocol = window.location.protocol
    targets.unshift(`${protocol}//${currentHost}:8547`) // Add at beginning for priority
    console.log(`[SetupWizard] Auto-detected network host: ${currentHost}:8547`)
  }

  // Add common fallbacks
  targets.push(
    'http://localhost:8000',
    'http://127.0.0.1:8547',
    'http://127.0.0.1:8000',
    'http://192.168.1.1:8547',
    'http://192.168.1.100:8547',
    'http://10.0.0.1:8547'
  )

  return targets
}

const scanTargets = buildScanTargets()

const appVersion = ref('')

onMounted(async () => {
  console.log('[SetupWizard] Component mounted, isTauri:', isTauri)

  // Get app version from Tauri backend or fallback
  if (isTauri && invoke) {
    try {
      appVersion.value = await invoke<string>('get_app_version')
    } catch (e) {
      appVersion.value = '1.0.10'
    }
  } else {
    // Browser mode - use env var or default
    appVersion.value = import.meta.env.VITE_APP_VERSION || '1.0.10'
  }

  console.log('[SetupWizard] Starting server discovery...')
  discoverServers()
})

async function discoverServers() {
  isScanning.value = true
  discoveredServers.value = []

  const total = scanTargets.length
  let checked = 0

  for (const targetUrl of scanTargets) {
    currentScanUrl.value = targetUrl

    try {
      const startTime = Date.now()
      console.log(`[SetupWizard] Testing server: ${targetUrl}`)

      const response = await axios.get(`${targetUrl}/api/health`, {
        timeout: 2000,
        headers: {
          'Accept': 'application/json'
        }
      })

      const responseTime = Date.now() - startTime

      if (response.status === 200) {
        console.log(`[SetupWizard] ✓ Server found: ${targetUrl}`, response.data)
        discoveredServers.value.push({
          url: targetUrl,
          name: response.data?.name || 'EvolveApp Server',
          status: 'online',
          version: response.data?.version,
          responseTime
        })
      }
    } catch (error: any) {
      console.log(`[SetupWizard] ✗ Server not available: ${targetUrl}`, error.message)
    }

    checked++
    scanProgress.value = (checked / total) * 100

    // Small delay to prevent overwhelming the network
    await new Promise(resolve => setTimeout(resolve, 100))
  }

  isScanning.value = false

  // Auto-select first server if found
  if (discoveredServers.value.length > 0) {
    selectedServer.value = discoveredServers.value[0]

    // Auto-connect to production server if it's discovered
    const productionServer = discoveredServers.value.find(s => s.url === 'https://evolvepreneuriq.app')
    if (productionServer) {
      selectedServer.value = productionServer
      // Auto-connect after short delay
      setTimeout(() => {
        testAndConnect()
      }, 500)
    }
  }
}

function selectServer(server: Server) {
  selectedServer.value = server
}

async function testAndConnect() {
  if (!selectedServer.value) return

  isConnecting.value = true

  try {
    const response = await axios.get(`${selectedServer.value.url}/api/health`)

    if (response.status === 200) {
      connectedServerName.value = selectedServer.value.name
      finalServerUrl.value = selectedServer.value.url

      // Save to localStorage
      localStorage.setItem('api_url', selectedServer.value.url)
      localStorage.setItem('server_name', selectedServer.value.name)
      localStorage.setItem('setup_completed', 'true')

      // Update axios base URL
      axios.defaults.baseURL = selectedServer.value.url

      currentStep.value = 'success'
    }
  } catch (error) {
    alert('Failed to connect to server. Please try again or enter a different URL.')
  } finally {
    isConnecting.value = false
  }
}

async function testManualUrl() {
  if (!manualUrl.value) {
    urlError.value = 'Please enter a server URL'
    return
  }

  // Basic URL validation
  try {
    new URL(manualUrl.value)
  } catch {
    urlError.value = 'Please enter a valid URL (e.g., http://localhost:8547)'
    return
  }

  isConnecting.value = true
  urlError.value = ''

  try {
    const response = await axios.get(`${manualUrl.value}/api/health`, {
      timeout: 5000
    })

    if (response.status === 200) {
      connectedServerName.value = response.data?.name || 'EvolveApp Server'
      finalServerUrl.value = manualUrl.value

      // Save to localStorage
      localStorage.setItem('api_url', manualUrl.value)
      localStorage.setItem('server_name', connectedServerName.value)
      localStorage.setItem('setup_completed', 'true')

      // Update axios base URL
      axios.defaults.baseURL = manualUrl.value

      currentStep.value = 'success'
    }
  } catch (error: any) {
    if (error.code === 'ECONNABORTED') {
      urlError.value = 'Connection timeout. Please check the URL and try again.'
    } else if (error.response) {
      urlError.value = `Server error: ${error.response.status}`
    } else {
      urlError.value = 'Could not connect to server. Please check the URL and make sure the server is running.'
    }
  } finally {
    isConnecting.value = false
  }
}

function finishSetup() {
  // Emit event or navigate to main app
  window.location.reload()
}
</script>

<style scoped>
.setup-wizard {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 2rem;
}

.wizard-container {
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  max-width: 600px;
  width: 100%;
  padding: 3rem;
}

.wizard-header {
  text-align: center;
  margin-bottom: 3rem;
}

.wizard-header h1 {
  font-size: 2.5rem;
  color: #333;
  margin-bottom: 0.5rem;
}

.subtitle {
  color: #666;
  font-size: 1.1rem;
}

.version {
  color: #999;
  font-size: 0.85rem;
  margin-top: 0.5rem;
}

.wizard-step {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.icon-container {
  display: flex;
  justify-content: center;
  margin-bottom: 1rem;
}

.scan-icon,
.server-icon {
  width: 80px;
  height: 80px;
  color: #667eea;
}

.rotating {
  animation: rotate 2s linear infinite;
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

h2 {
  text-align: center;
  color: #333;
  font-size: 1.8rem;
  margin: 0;
}

.scanning-status {
  text-align: center;
}

.progress-bar {
  height: 8px;
  background: #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 1rem;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea, #764ba2);
  transition: width 0.3s ease;
}

.server-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-height: 300px;
  overflow-y: auto;
}

.server-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem;
  border: 2px solid #e0e0e0;
  border-radius: 12px;
  background: white;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
}

.server-item:hover {
  border-color: #667eea;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
}

.server-item.selected {
  border-color: #667eea;
  background: #f7f9ff;
}

.server-info h3 {
  margin: 0 0 0.5rem 0;
  color: #333;
  font-size: 1.2rem;
}

.server-url {
  color: #666;
  font-size: 0.9rem;
  margin: 0.25rem 0;
  font-family: monospace;
}

.server-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #999;
  font-size: 0.85rem;
  margin: 0.5rem 0 0 0;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #4caf50;
}

.check-icon {
  width: 32px;
  height: 32px;
  color: #667eea;
  flex-shrink: 0;
}

.no-servers {
  text-align: center;
  padding: 2rem;
  color: #666;
}

.no-servers .hint {
  color: #999;
  font-size: 0.9rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-weight: 600;
  color: #333;
}

.form-group input {
  padding: 0.75rem 1rem;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.form-group input:focus {
  outline: none;
  border-color: #667eea;
}

.hint {
  color: #999;
  font-size: 0.85rem;
  margin: 0;
}

.error-message {
  color: #f44336;
  font-size: 0.9rem;
  margin: 0;
}

.preset-servers {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.preset-servers .label {
  font-weight: 600;
  color: #666;
  font-size: 0.9rem;
  margin: 0;
}

.preset-btn {
  padding: 0.75rem 1rem;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background: white;
  color: #333;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.preset-btn:hover {
  border-color: #667eea;
  background: #f7f9ff;
}

.wizard-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.btn-primary,
.btn-secondary {
  flex: 1;
  padding: 1rem 2rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: white;
  color: #667eea;
  border: 2px solid #667eea;
}

.btn-secondary:hover {
  background: #f7f9ff;
}

.wizard-step.success {
  text-align: center;
}

.success-icon {
  display: flex;
  justify-content: center;
  margin-bottom: 2rem;
}

.success-icon svg {
  width: 100px;
  height: 100px;
  color: #4caf50;
}

.connection-details {
  background: #f7f9ff;
  padding: 1.5rem;
  border-radius: 8px;
  margin: 2rem 0;
}

.connection-details p {
  margin: 0.5rem 0;
  color: #333;
}

.connection-details strong {
  color: #667eea;
}
</style>
