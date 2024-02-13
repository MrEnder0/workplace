# Workplace

## Description

This allows you to remotely shutdown specific processes on devices on the same network.

## Instalation

* Download the lastest workplace server executable from the releases page and put it in its startup.
* After you setup your server get all your computers you want to controll the workplace client on them and put them in startup, make sure you also create a "server_ip" file contaning solely the local private ip of the server.

> [!IMPORTANT]
> This following command is required if you have your firewall enabled on your Windows host; this will allow the server to receive requests on port 3012, make sure to run it as an administrator.

```powershell
netsh advfirewall firewall add rule name= "Workplace Server" dir=in action=allow protocol=TCP localport=3012
```