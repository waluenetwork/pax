import asyncio
import json
import random
from datetime import datetime
from typing import List
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class DataItem:
    def __init__(self, id: int, title: str):
        self.id = id
        self.title = title
        self.value = random.uniform(10, 1000)
        self.trend = self.generate_trend()
        self.color = self.generate_color()
        self.timestamp = datetime.now().strftime("%H:%M:%S")
    
    def generate_trend(self):
        change = random.uniform(-50, 50)
        if change > 10:
            return f"↑ +{change:.1f}%"
        elif change < -10:
            return f"↓ {change:.1f}%"
        else:
            return f"→ {change:.1f}%"
    
    def generate_color(self):
        return random.choice(["green", "red", "blue"])
    
    def to_dict(self):
        return {
            "id": self.id,
            "title": self.title,
            "value": self.value,
            "trend": self.trend,
            "color": self.color,
            "timestamp": self.timestamp
        }

class ConnectionManager:
    def __init__(self):
        self.active_connections: List[WebSocket] = []

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        self.active_connections.append(websocket)

    def disconnect(self, websocket: WebSocket):
        self.active_connections.remove(websocket)

    async def broadcast(self, data: str):
        for connection in self.active_connections:
            try:
                await connection.send_text(data)
            except:
                self.disconnect(connection)

manager = ConnectionManager()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await manager.connect(websocket)
    try:
        message_count = 0
        while True:
            await asyncio.sleep(2)
            message_count += 1
            
            if message_count % 5 == 0:
                template_content = generate_dynamic_template(message_count)
                template_message = {
                    "Template": template_content
                }
                print(f"📨 Sending FIXED FINAL template message #{message_count}")
                print(f"🔧 Template preview: {template_content[:200]}...")
                await manager.broadcast(json.dumps(template_message))
            else:
                data_items = [
                    DataItem(1, "Revenue"),
                    DataItem(2, "Active Users"), 
                    DataItem(3, "Conversion Rate"),
                    DataItem(4, "Server Load"),
                    DataItem(5, "Response Time"),
                ]
                
                data_message = {
                    "Data": [item.to_dict() for item in data_items]
                }
                print(f"📊 Sending data message #{message_count}")
                await manager.broadcast(json.dumps(data_message))
                
    except WebSocketDisconnect:
        manager.disconnect(websocket)

def generate_dynamic_template(count: int) -> str:
    """Generate dynamic Pax template with PROPERLY FIXED syntax and correct ID selectors"""
    colors = ["rgb(50, 50, 200)", "rgb(200, 50, 50)", "rgb(50, 200, 50)"]
    color = colors[count % len(colors)]
    
    return f"""<Group x=50% y=50% width=80% height=60%>
    <Text x=50% y=20% text="Dynamic SSR Dashboard #{count}" id=title/>
    <Text x=50% y=40% text={{connection_status}} id=status/>
    <Text x=50% y=60% text={{last_update}} id=update/>
    <Rectangle fill={color} width=100% height=100%/>
</Group>

@settings {{
    @mount: on_mount,
        style: {{
            font: Font::Web("Roboto", "", FontStyle::Normal, FontWeight::Normal)
            font_size: 24px
            fill: WHITE
            align_vertical: TextAlignVertical::Center
            align_horizontal: TextAlignHorizontal::Center
        }}
    }}
        style: {{
            font: Font::Web("Roboto", "", FontStyle::Normal, FontWeight::Normal)
            font_size: 16px
            fill: WHITE
            align_vertical: TextAlignVertical::Center
            align_horizontal: TextAlignHorizontal::Center
        }}
    }}
        style: {{
            font: Font::Web("Roboto", "", FontStyle::Normal, FontWeight::Normal)
            font_size: 14px
            fill: WHITE
            align_vertical: TextAlignVertical::Center
            align_horizontal: TextAlignHorizontal::Center
        }}
    }}
}}"""

@app.get("/")
async def root():
    return {"message": "SSR WebSocket Server for Pax Dashboard - FIXED FINAL VERSION"}

@app.get("/healthz")
async def healthz():
    return {"status": "ok"}
