# Workplace

## Description

This allows you to remotely shutdown specific processes on devices on the same network.

## Installation

* Download the latest workplace server executable from the releases page and put it in startup applications.
* Add a firewall rule shown below for the server pc to allow it to communicate with the clients.

> [!IMPORTANT]
> This following command is required if you have your firewall enabled on your Windows host; this will allow the server to receive requests on port 3012, make sure to run it as an administrator.
> ```powershell
> netsh advfirewall firewall add rule name= "Workplace Server" dir=in action=allow protocol=TCP localport=3012
> ```

* Now that you have set up your server get all the computers you want to manage and create a folder called "WorkPlace" on the C drive, then put the workplace client executable in this folder. If you want you can also just put the client executable directly in the startup folder, though the workplace folder though will store things like logs.
* Restart all the computers and allow the clients to start up, this is recommended because they client may have updated / downgraded versions to match the server it connected to.

> [!IMPORTANT]
> As of 0.2.0 the clients and servers both no longer require the server_ip.dat files and they can be deleted from all systems when updating to a supported version.
