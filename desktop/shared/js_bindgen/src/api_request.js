const request_path = (path) => `https://vulkan-8vfs.shuttle.app/v1/${path}`;
// const request_path = (path) => `http://127.0.0.1:4576/v1/${path}`;

async function post(payload, endpoint) {
  try {
    const url = request_path(endpoint);
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const json = await response.json();
    console.log(json);
  } catch (error) {
    console.error(error.message);
  }
}

async function get( endpoint) {
  try {
    const url = request_path(endpoint);
    const response = await fetch("http://127.0.0.1:4576/v1/health", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const json = await response.text();
    console.log(json);
  } catch (error) {
    console.error(error.message);
  }
}

post(
  {
    "email": "test@mailer.com",
    "first_name": "test",
    "last_name": "test",
    "password": "test",
  },
  "users/register"
);

// get("health");
