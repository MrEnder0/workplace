# Workplace

## Description

This allows you to remotely shutdown specific processes on devices on the same network.

## Installation

* Download the lastest workplace server executable from the releases page and put it in startup applications.
* Add a firewall rule shown below for the server pc to allow it to communcate with the clients.

> [!IMPORTANT]
> This following command is required if you have your firewall enabled on your Windows host; this will allow the server to receive requests on port 3012, make sure to run it as an administrator.
> ```powershell
> netsh advfirewall firewall add rule name= "Workplace Server" dir=in action=allow protocol=TCP localport=3012
> ```

* Now that you have setup your server get all your computers you want to control the workplace client executable, put this executable in the startup folder for the client computers
* Create a file called "server_ip.dat" and place this in the directory "C:\ProgramData", If your clients windows instalation is not on the C drive please create an issue so we can communcate a solution.
* Inside your server_ip file put the private ip of the server computer.

> [!NOTE]
> Make sure your server_ip file has the .dat extension and is located in the C:\ProgramData directory, also if no server ip file is found the client will continue with seemingly normal operation but with a default set ip of localhost/127.0.0.1.
