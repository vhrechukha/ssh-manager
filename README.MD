# 🌐 SSH Manager

The SSH Manager is a tool designed to help manage SSH connections and configurations. It provides functionality for organizing and connecting to remote servers via SSH.

## Connecting via SSH Configuration

To install SSH Manager, you can use Homebrew by running the following commands in your terminal, follow these steps:

```bash
brew tap vhrechukha/ssh-manager
brew install ssh-manager
```

## Features

1. **Connection Management**: The SSH Manager allows users to manage SSH connection configurations, including the host, path to the key file, and alias.
2. **Easy SSH Connection Switch**: The SSH Manager provides a convenient method to switch between different connections for a single host. Existing configurations with the same host are automatically removed to avoid conflicts.
3. **User-friendly Interface**: The SSH Manager offers a user-friendly interface with intuitive controls and navigation, making it easy to manage SSH connections.
4. **Multilingual Support**: The SSH Manager supports multiple languages, including English and Ukrainian.

## Usage

1. **Add SSH Configuration**: Users can add a new SSH configuration by providing the necessary details such as the host address, global path, and alias.
2. **Remove SSH Configuration**: Users can remove an added SSH configuration.
3. **Connect via SSH Configuration**: Utilize the *Use Configuration* feature to connect to previously added configurations with auto-removing existing configurations for the same host. The SSH Manager automatically handles the management of configurations, allowing you to seamlessly switch between different SSH connections without any hassle.
4. **Switch Language**: Users can change the language interface to their preferred language.

<iframe width="560" height="315" src="https://www.youtube.com/embed/TtXrencNSL0" frameborder="0" allowfullscreen></iframe>

**Note: Global Path Format**

When specifying the global path in the SSH Manager's configuration, the path should be relative to the root directory of your Mac. For example, if your username is `user123` and the path to the configuration file is `/Users/user123/.ssh/id_of_the_config`, you should enter the path as `.ssh/id_of_the_config` in the global path configuration.

The global path format allows for easier management of SSH configurations, as it automatically references the user's home directory without requiring the full absolute path.

---

The SSH Manager is a valuable tool for individuals or teams that frequently work with SSH connections, offering an efficient and secure way to manage and access remote servers.
