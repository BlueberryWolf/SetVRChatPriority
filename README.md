# SetVRChatPriority
EasyAntiCheat prevents us from changing the priority.  Let's fix that!

SetVRChatPriority is a program you run once before VRChat, and it permanently sets VRChat's priority on every startup!
![Preview Image](https://cdn.discordapp.com/attachments/924219614257348650/1006908690349170698/unknown.png)

The new VRChat update includes EasyAntiCheat, which prevents you from changing the priority of VRChat.exe through the task manager.

SetVRChatPriority sets a registry key `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\VRChat.exe\PerfOptions\VRChat.exe\CpuPriorityClass` to force VRChat.exe to start with a set priority.

SetVRChatPriority **must be ran before VRChat is started.**

Latest Download: https://github.com/BlueberryWolf/SetVRChatPriority/releases/latest
