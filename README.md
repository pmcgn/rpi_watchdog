# Welcome to rpi-watchdog
 rpi-watchdog is a simple way to make use of the Raspberry Pi's hardware watchdog. The Idea is to run this tool inside a docker container. If the system crashes or a monitored URL (optional) does not resond with HTTP 200 OK, the watchdog will trigger a reset.

# The Docker Image
tbd

# How dows it work?
The watchdog inside the rasperry Pi is a separate hardware unit inside each Pi. After it has been activated, it requires a write command to /dev/watchdog. If the command does not come within a 15s timeframe, the watchdog will reset the Pi. As the watchdog is realized in hardware, it is able to trigger a reset, even if the operating system stops responding.
With the ability to monitor a website (e.g. smarthome system running in another docker container), the watchdog can also be used to restart the Pi, if a single container has crashed (this will be improved, but for now it's an effective way to ensure availability of a particular system).