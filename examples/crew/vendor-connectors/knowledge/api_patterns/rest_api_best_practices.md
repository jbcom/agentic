# REST API Best Practices for Connector Generation

This document outlines best practices for generating HTTP API connectors.

## Client Architecture

### Use httpx for HTTP Requests

```python
import httpx
from typing import Optional

class APIClient:
    def __init__(
        self,
        api_key: str,
        base_url: str = "https://api.example.com",
        timeout: float = 30.0,
    ):
        self.api_key = api_key
        self.base_url = base_url
        self._client = httpx.AsyncClient(
            base_url=base_url,
            timeout=timeout,
            headers={"Authorization": f"Bearer {api_key}"},
        )
    
    async def __aenter__(self):
        return self
    
    async def __aexit__(self, *args):
        await self._client.aclose()
```

### Use Pydantic for Data Validation

```python
from pydantic import BaseModel, Field
from typing import Literal

class CreateRequest(BaseModel):
    name: str = Field(..., min_length=1, max_length=255)
    type: Literal["text", "image", "video"]
    options: dict[str, str] = Field(default_factory=dict)

class CreateResponse(BaseModel):
    id: str
    status: Literal["pending", "completed", "failed"]
    created_at: str
```

## Error Handling

### Custom Exceptions

```python
class APIError(Exception):
    """Base exception for API errors."""
    pass

class AuthenticationError(APIError):
    """Raised when authentication fails."""
    pass

class RateLimitError(APIError):
    """Raised when rate limit is exceeded."""
    def __init__(self, retry_after: int):
        self.retry_after = retry_after
        super().__init__(f"Rate limited. Retry after {retry_after}s")

class ValidationError(APIError):
    """Raised when request validation fails."""
    pass
```

### Error Response Handling

```python
async def _request(
    self,
    method: str,
    endpoint: str,
    **kwargs
) -> dict:
    try:
        response = await self._client.request(method, endpoint, **kwargs)
        response.raise_for_status()
        return response.json()
    except httpx.HTTPStatusError as e:
        if e.response.status_code == 401:
            raise AuthenticationError("Invalid API key")
        elif e.response.status_code == 429:
            retry_after = int(e.response.headers.get("Retry-After", 60))
            raise RateLimitError(retry_after)
        elif e.response.status_code == 422:
            raise ValidationError(f"Validation error: {e.response.text}")
        else:
            raise APIError(f"HTTP {e.response.status_code}: {e.response.text}")
    except httpx.RequestError as e:
        raise APIError(f"Request failed: {str(e)}")
```

## Authentication Patterns

### API Key (Header)

```python
headers = {"Authorization": f"Bearer {api_key}"}
```

### API Key (Query Parameter)

```python
params = {"api_key": api_key}
```

### OAuth2

```python
from authlib.integrations.httpx_client import AsyncOAuth2Client

client = AsyncOAuth2Client(
    client_id=client_id,
    client_secret=client_secret,
    token_endpoint="https://api.example.com/oauth/token",
)
await client.fetch_token()
```

## Rate Limiting

### Implement Rate Limiter

```python
import asyncio
from datetime import datetime, timedelta

class RateLimiter:
    def __init__(self, requests_per_second: int):
        self.requests_per_second = requests_per_second
        self.min_interval = 1.0 / requests_per_second
        self.last_request_time = None
    
    async def acquire(self):
        if self.last_request_time is None:
            self.last_request_time = datetime.now()
            return
        
        elapsed = (datetime.now() - self.last_request_time).total_seconds()
        if elapsed < self.min_interval:
            await asyncio.sleep(self.min_interval - elapsed)
        
        self.last_request_time = datetime.now()
```

### Usage

```python
class APIClient:
    def __init__(self, api_key: str, requests_per_second: int = 10):
        self.rate_limiter = RateLimiter(requests_per_second)
        # ...
    
    async def _request(self, method: str, endpoint: str, **kwargs):
        await self.rate_limiter.acquire()
        # ... make request
```

## Pagination Patterns

### Offset-Based Pagination

```python
async def list_items(
    self,
    limit: int = 100,
    offset: int = 0,
) -> list[Item]:
    response = await self._request(
        "GET",
        "/items",
        params={"limit": limit, "offset": offset},
    )
    return [Item(**item) for item in response["items"]]
```

### Cursor-Based Pagination

```python
from typing import Optional

async def list_items(
    self,
    limit: int = 100,
    cursor: Optional[str] = None,
) -> tuple[list[Item], Optional[str]]:
    params = {"limit": limit}
    if cursor:
        params["cursor"] = cursor
    
    response = await self._request("GET", "/items", params=params)
    items = [Item(**item) for item in response["items"]]
    next_cursor = response.get("next_cursor")
    return items, next_cursor

# Usage
async def get_all_items(self) -> list[Item]:
    all_items = []
    cursor = None
    
    while True:
        items, cursor = await self.list_items(cursor=cursor)
        all_items.extend(items)
        if cursor is None:
            break
    
    return all_items
```

## File Upload/Download

### Upload Files

```python
async def upload_file(
    self,
    file_path: str,
    file_type: str,
) -> UploadResponse:
    with open(file_path, "rb") as f:
        files = {"file": (file_path, f, file_type)}
        response = await self._request(
            "POST",
            "/upload",
            files=files,
        )
    return UploadResponse(**response)
```

### Download Files

```python
async def download_file(
    self,
    file_id: str,
    output_path: str,
) -> None:
    response = await self._client.get(f"/files/{file_id}")
    response.raise_for_status()
    
    with open(output_path, "wb") as f:
        f.write(response.content)
```

## WebSocket Support

```python
import websockets
import json

async def subscribe_to_updates(
    self,
    on_message: callable,
) -> None:
    uri = f"wss://api.example.com/subscribe?api_key={self.api_key}"
    
    async with websockets.connect(uri) as websocket:
        while True:
            message = await websocket.recv()
            data = json.loads(message)
            await on_message(data)
```

## Testing Patterns

### Mock HTTP Responses

```python
import pytest
from pytest_httpx import HTTPXMock

@pytest.mark.asyncio
async def test_create_item(httpx_mock: HTTPXMock):
    httpx_mock.add_response(
        method="POST",
        url="https://api.example.com/items",
        json={"id": "123", "status": "completed"},
    )
    
    async with APIClient(api_key="test") as client:
        result = await client.create_item(name="Test")
        assert result.id == "123"
```

### Test Error Handling

```python
@pytest.mark.asyncio
async def test_rate_limit_error(httpx_mock: HTTPXMock):
    httpx_mock.add_response(
        status_code=429,
        headers={"Retry-After": "60"},
    )
    
    async with APIClient(api_key="test") as client:
        with pytest.raises(RateLimitError) as exc_info:
            await client.create_item(name="Test")
        assert exc_info.value.retry_after == 60
```

## Framework Tool Generation

### CrewAI Tools

```python
from crewai_tools import Tool

def get_tools(framework: str = "crewai"):
    if framework == "crewai":
        return [
            Tool(
                name="create_item",
                description="Create a new item with the given name",
                func=lambda name: asyncio.run(
                    APIClient(api_key=os.getenv("API_KEY")).create_item(name)
                ),
            ),
        ]
```

### LangGraph Tools

```python
from langchain_core.tools import StructuredTool

def get_tools(framework: str = "langgraph"):
    if framework == "langgraph":
        return [
            StructuredTool.from_function(
                name="create_item",
                description="Create a new item",
                func=lambda name: asyncio.run(
                    APIClient(api_key=os.getenv("API_KEY")).create_item(name)
                ),
            ),
        ]
```

## Documentation Standards

### Client Docstrings

```python
class APIClient:
    """HTTP client for Example API.
    
    This client provides async methods for interacting with the Example API.
    It handles authentication, rate limiting, and error handling automatically.
    
    Args:
        api_key: Your API key from https://example.com/settings
        base_url: API base URL (default: https://api.example.com)
        timeout: Request timeout in seconds (default: 30.0)
        
    Example:
        >>> async with APIClient(api_key="your-key") as client:
        ...     result = await client.create_item("Test")
        ...     print(result.id)
    """
```

### Method Docstrings

```python
async def create_item(
    self,
    name: str,
    type: str = "text",
) -> CreateResponse:
    """Create a new item.
    
    Args:
        name: Item name (1-255 characters)
        type: Item type (text, image, or video)
        
    Returns:
        CreateResponse with item ID and status
        
    Raises:
        AuthenticationError: If API key is invalid
        ValidationError: If parameters are invalid
        RateLimitError: If rate limit is exceeded
        APIError: For other API errors
        
    Example:
        >>> result = await client.create_item("My Item", type="text")
        >>> print(result.id)
    """
```
