# Scheduler Telegram Bot

This bot is designed to display schedules. It provides a user-friendly interface to select your group and view the schedule for today, tomorrow, yesterday, or the entire week.

## Bot Functionality

The bot offers the following commands and features:

*   **/start**: Initiates interaction with the bot. Prompts you to select your study group using inline buttons.
*   **/schedule**: Displays the main schedule viewing buttons after a group has been selected.
*   **"Today"**: Shows the schedule for the current day for the selected group.
*   **"Tomorrow"**: Shows the schedule for the next day for the selected group.
*   **"Yesterday"**: Displays the schedule for the previous day for the selected group.
*   **"Week"**: Presents the schedule for the entire current week for the selected group.
*   **Group Selection**: Upon first launch or using the `/start` command, the bot will ask you to choose your group (e.g., "1A", "1B", "2A", "2B") via interactive buttons.

## Running the Bot

To run the bot, you will need to have either **Docker** or **Podman** installed on your system. You will also need a `BOT_TOKEN` from BotFather in Telegram.

### 1. Preparation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/TOwInOK/scheduler.git
    cd scheduler
    ```
2.  **Create an `.env` file:**
    In the project's root directory, create a file named `.env` and add your bot token to it:
    ```/Users/towinok/Documents/rustdir/sheduller/.env
    BOT_TOKEN=YOUR_SECRET_BOT_TOKEN
    ```
    Replace `YOUR_SECRET_BOT_TOKEN` with the actual token you obtained from BotFather.

### 2. Pulling the Pre-built Image

You can pull the latest pre-built image directly from GitHub Container Registry (GHCR):

```bash
# For Docker:
docker pull ghcr.io/towinok/scheduler:latest

# For Podman:
podman pull ghcr.io/towinok/scheduler:latest
```

### 3. Running with Docker Compose (Recommended)

This is the most recommended way to manage the bot, simplifying the build/pull and run process.

1.  **Update `docker-compose.yml` to use the pre-built image:**
    Modify your `docker-compose.yml` to specify the `image` instead of `build` if you prefer to use the pre-built image.
    ```/Users/towinok/Documents/rustdir/sheduller/docker-compose.yml
    version: '3.8'

    services:
      scheduler:
        image: ghcr.io/towinok/scheduler:latest # Specify the image to pull
        container_name: scheduler_bot
        environment:
          - BOT_TOKEN=${BOT_TOKEN}
        restart: on-failure
    ```
    Then, run:
    ```bash
    docker compose up -d
    ```

2.  **To build the image locally (from source) with Docker Compose:**
    Ensure your `docker-compose.yml` specifies `build: .` (as provided initially).
    ```bash
    docker compose up --build -d
    ```
    This command will build the image (if not already created or if there are changes) and run the container in detached mode.

3.  **Stop the container:**
    ```bash
    docker compose down
    ```

### 4. Running with Docker or Podman (Manually)

If you prefer to manage images and containers manually:

1.  **Run the container using the pre-built image:**
    ```bash
    # For Docker:
    docker run -d --name scheduler_bot -e BOT_TOKEN="${BOT_TOKEN}" ghcr.io/towinok/scheduler:latest

    # For Podman:
    podman run -d --name scheduler_bot -e BOT_TOKEN="${BOT_TOKEN}" ghcr.io/towinok/scheduler:latest
    ```
    **Important:** Ensure that the `BOT_TOKEN` environment variable is available in your current shell before executing the `run` command (e.g., `export BOT_TOKEN="YOUR_SECRET_BOT_TOKEN"` or use `$(cat .env | grep BOT_TOKEN | cut -d '=' -f2)`).

2.  **To build and run the image locally (from source):**
    First, build the image:
    ```bash
    # For Docker:
    docker build . -t scheduler_bot:latest

    # For Podman:
    podman build . -t scheduler_bot:latest
    ```
    Then, run it:
    ```bash
    # For Docker:
    docker run -d --name scheduler_bot -e BOT_TOKEN="${BOT_TOKEN}" scheduler_bot:latest

    # For Podman:
    podman run -d --name scheduler_bot -e BOT_TOKEN="${BOT_TOKEN}" scheduler_bot:latest
    ```

3.  **Stopping the container:**
    ```bash
    # For Docker:
    docker stop scheduler_bot

    # For Podman:
    podman stop scheduler_bot
    ```
