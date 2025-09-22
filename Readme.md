# Sheduller Telegram Bot

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

### 2. Running with Docker Compose

This is the recommended method as it simplifies the build and run process.

1.  **Build and start the container:**
    ```bash
    docker compose up --build -d
    ```
    This command will build the image (if not already built or if there are changes) and run the container in detached mode.

2.  **Stop the container:**
    ```bash
    docker compose down
    ```

### 3. Running with Docker or Podman (Manually)

If you prefer to manage images and containers manually:

1.  **Build the Docker/Podman image:**
    ```bash
    # For Docker:
    docker build . -t sheduller_bot:latest

    # For Podman:
    podman build . -t sheduller_bot:latest
    ```

2.  **Run the container:**
    ```bash
    # For Docker:
    docker run -d --name sheduller_instance -e BOT_TOKEN="${BOT_TOKEN}" sheduller_bot:latest

    # For Podman:
    podman run -d --name sheduller_instance -e BOT_TOKEN="${BOT_TOKEN}" sheduller_bot:latest
    ```
    **Important:** Ensure that the `BOT_TOKEN` environment variable is available in your current shell before running the `run` command (e.g., `export BOT_TOKEN="YOUR_SECRET_BOT_TOKEN"` or use `$(cat .env | grep BOT_TOKEN | cut -d '=' -f2)`).

3.  **Stop the container:**
    ```bash
    # For Docker:
    docker stop sheduller_instance

    # For Podman:
    podman stop sheduller_instance
    ```
