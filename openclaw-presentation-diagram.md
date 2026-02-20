# OpenClaw Architecture - Presentation Diagram

## 🎯 High-Level System Architecture

```
                           USERS & CHANNELS
    ┌─────────────────────────────────────────────────────────────────────────┐
    │  📱WhatsApp  📞Telegram  💬Slack  🎮Discord  📧Email  🍎iMessage  💼Teams │
    └─────────────────────┬───────────────────────────────────────────────────┘
                          │ Protocol-Specific Connections
                          │ (Baileys, grammY, Bolt, discord.js, etc.)
                          ▼
    ┌─────────────────────────────────────────────────────────────────────────┐
    │                          OPENCLAW GATEWAY                               │
    │                        🌐 ws://127.0.0.1:18789                        │
    │                                                                         │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │
    │  │   CHANNELS   │  │ ACCESS CTRL  │  │ SESSIONS     │  │   TOOLS     │ │
    │  │              │  │              │  │              │  │             │ │
    │  │ • Normalize  │  │ • DM Pairing │  │ • Multi-Agent│  │ • Sandbox   │ │
    │  │ • Route      │  │ • Allowlists │  │ • Trees      │  │ • Browser   │ │
    │  │ • Transform  │  │ • Auth Tokens│  │ • Queue      │  │ • FileSystem│ │
    │  └──────────────┘  └──────────────┘  └──────────────┘  └─────────────┘ │
    │                                                                         │
    │              📡 WebSocket API: REQ/RES + EVENT STREAMING               │
    └─────────────────────┬───────────────────────────────────────────────────┘
                          │ Bidirectional Real-time Communication
                          ▼
    ┌─────────────────────────────────────────────────────────────────────────┐
    │                           AI PROCESSING LAYER                           │
    │                                                                         │
    │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │
    │  │  PI-AGENT    │  │  MODELS      │  │   SKILLS     │  │ AUTOMATION  │ │
    │  │              │  │              │  │              │  │             │ │
    │  │ • RPC        │  │ • Claude     │  │ • Registry   │  │ • Cron Jobs │ │
    │  │ • Streaming  │  │ • GPT        │  │ • Hot Reload │  │ • Webhooks  │ │
    │  │ • Tools      │  │ • Failover   │  │ • ClawHub    │  │ • PubSub    │ │
    │  └──────────────┘  └──────────────┘  └──────────────┘  └─────────────┘ │
    └─────────────────────┬───────────────────────────────────────────────────┘
                          │ Device Commands & Responses
                          ▼
    ┌─────────────────────────────────────────────────────────────────────────┐
    │                          COMPANION DEVICES                              │
    │  📱iOS Node   🤖Android   🖥️macOS App   🌐Control UI   ⌨️CLI Tools     │
    │  • Canvas     • Camera    • Menu Bar    • Dashboard   • Direct Cmds    │
    │  • Voice      • Screen    • Voice Wake  • WebChat     • Scripts        │
    │  • Location   • Notify    • Overlay     • Admin       • Debugging      │
    └─────────────────────────────────────────────────────────────────────────┘
```

## 🔄 Message Flow (Your Analysis Confirmed!)

```
┌─────┐     ┌──────────┐     ┌──────────┐     ┌─────────┐     ┌──────────┐
│USER │────▶│ CHANNEL  │────▶│ GATEWAY  │────▶│ AGENT   │────▶│ RESPONSE │
│     │     │ PROTOCOL │     │ ROUTER   │     │ AI LOOP │     │ DELIVERY │
└─────┘     └──────────┘     └──────────┘     └─────────┘     └──────────┘
             Baileys WS       Normalize        Claude API       Multi-cast
             grammY HTTP      InboundEnvelope  Tool Exec        All Channels
             Discord.js       Access Check     Stream Back      + UI Updates
```

**Step-by-Step (Your Understanding ✅)**:
1. 📱 **Phone** → WhatsApp servers → **Baileys** (persistent WS)
2. **Baileys** fires `onMessage` → Gateway normalizes to `InboundEnvelope`  
3. **Access Control**: pairing ✅, allowlist ✅
4. **Route to Agent** → AI processing loop → **Tool execution**
5. **Response** via same Baileys WS connection
6. **Live Events** pushed to all WebSocket clients (Control UI sees activity)

## 🔐 Authentication & Security Flow

```
    NEW DEVICE PAIRING SEQUENCE
    ────────────────────────────────
    
    Device                 Gateway                    Admin
      │                      │                        │
      │── req:connect ──────▶│                        │
      │                      │── generate code ABC123 │
      │◄─── challenge ───────│                        │
      │                      │                        │
      │                      │◄─── approve ABC123 ────│
      │                      │                        │  
      │── signed response ──▶│                        │
      │                      │── store allowlist ────▶│
      │◄─── device token ────│                        │
      │                      │                        │
      │── authenticated ────▶│                        │
           commands          │                        │
           
    ONGOING AUTHENTICATION
    ─────────────────────────
    
    Every WebSocket connection requires:
    ✅ Device ID + Token (or fresh pairing)
    ✅ Gateway auth token/password  
    ✅ Channel-level authorization (allowlists, DM policy)
    ✅ Per-tool permissions (sandbox, elevated)
```

## ⚡ Smart Design Highlights for Interview

### 1. **Protocol Normalization** (Eliminates N×M Complexity)
```
❌ Without: 10 channels × 5 agents = 50 integrations
✅ With:    10 channels → 1 Gateway → N agents = 10 + N integrations

All channels produce: InboundEnvelope {channel, peer, text, sessionKey}
```

### 2. **WebSocket-First Real-Time Architecture**
```
Traditional:  Client ──polling──▶ Server (wasteful, delayed)
OpenClaw:     Client ◄──WS────▶ Server (efficient, instant)

Benefits:
• Live typing indicators
• Real-time tool execution updates  
• Multi-device sync
• Efficient resource usage
```

### 3. **Agent-to-Agent Communication**
```
Complex Task: "Analyze my codebase and create deployment plan"

Main Agent ──spawn──▶ Code Analyzer Agent ──results──▶ Main Agent
     │                                                      │
     └──spawn──▶ DevOps Agent ──deployment plan──▶ Main Agent

Each specialized agent has:
• Different model settings
• Specific tool permissions
• Isolated context
• Independent error handling
```

### 4. **Security Defense-in-Depth**
```
🔴 Network:    Loopback bind, Tailscale, TLS
🟡 Gateway:    Token auth, device pairing
🟠 Channel:    DM policies, group allowlists  
🟢 Agent:      Sandboxed tools, per-agent limits
🔵 Data:       Encrypted storage, session isolation
```

### 5. **Node Federation Pattern**
```
Lightweight Device Nodes:
📱 iOS/Android: Expose camera, voice, location
🖥️ macOS:      Add system commands, screen recording
🌐 Browser:     Remote CDP control

Benefits:
• Battery efficiency (AI runs on Gateway host)
• Unified API regardless of device location
• Secure command proxying
• Horizontal scaling
```

## 🎯 System Design Interview Questions You Can Answer

### **Q: How would you scale this to 1000+ users?**
**A**: Multi-tenant Gateway deployment:
- Docker containers per user/org
- Shared model API pool with rate limiting
- Redis for session state clustering
- Load balancer for WebSocket connections

### **Q: What happens if the Gateway crashes?**
**A**: Designed for resilience:
- Sessions persist to disk (survive restarts)
- Channels reconnect automatically
- Device tokens remain valid
- macOS app can supervise Gateway process

### **Q: How do you prevent prompt injection attacks?**
**A**: Defense-in-depth:
- Sandboxed tool execution (Docker isolation)
- Per-agent tool allowlists (deny dangerous tools)
- Modern models with instruction hardening
- Access control before intelligence
- Audit trails for all actions

### **Q: How would you add a new messaging platform?**
**A**: Plugin architecture:
```javascript
class NewChannelPlugin implements ChannelPlugin {
  onMessage(rawMessage) {
    return {
      channel: 'newplatform',
      peer: rawMessage.userId,
      text: rawMessage.content,
      sessionKey: `main:newplatform:dm:${rawMessage.userId}`
    }
  }
  
  sendMessage(envelope) {
    return this.platformAPI.send(envelope.peer, envelope.text)
  }
}
```

Your WhatsApp flow analysis is **architecturally perfect**! This system brilliantly solves the personal AI assistant problem while maintaining security, real-time performance, and extensibility.