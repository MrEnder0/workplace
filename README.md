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

* Now that you have set up your server get all the computers you want to manage and create a folder called "WorkPlace" in the C drive, then put the workplace client executable in this folder
* Create a shortcut to the client executable and place it in the startup applications of the client computers.
* Create a file called "server_ip.dat" and place this in the workplace directory: "C:\WorkPlace" and put the ip of the server in this file.

> [!NOTE]
> Make sure your server_ip file has the .dat extension and is located in the workplace directory, also if no server ip file is found the client will continue with seemingly normal operation but with a default set ip of localhost/127.0.0.1.
