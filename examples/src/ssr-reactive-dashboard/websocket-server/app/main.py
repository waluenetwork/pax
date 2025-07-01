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
                await manager.broadcast(json.dumps(data_message))
                
    except WebSocketDisconnect:
        manager.disconnect(websocket)

def generate_dynamic_template(count: int) -> str:
    """Generate BASIC Pax template without styling to avoid parsing errors"""
    colors = ["rgb(50, 50, 200)", "rgb(200, 50, 50)", "rgb(50, 200, 50)"]
    color = colors[count % len(colors)]
    
    return f"""<Group x=50% y=50% width=80% height=60%>
    <Text x=50% y=20% text="Dynamic SSR Dashboard #{count}"/>
    <Text x=50% y=40% text={{connection_status}}/>
    <Text x=50% y=60% text={{last_update}}/>
    <Rectangle fill={color} width=100% height=100%/>
</Group>"""

@app.get("/")
async def root():
    return {"message": "SSR WebSocket Server for Pax Dashboard"}

@app.get("/healthz")
async def healthz():
    return {"status": "ok"}
