# A cli tool made for pinging http endpoints.

*This is a personal tool so don't expect excellent maintenence.

## How to install

1. copy the git repo
```bash git clone https://github.com/dorian3343/CrabPing```
2. Build it with cargo build --release.

3. On linux, find the binary in target/release.
- Add it to your path
```bash sudo cp CrabPing /usr/local/bin/```

4. On windows, idk I don't use windows :P

## How to use

```bash CrabPing [HttpAdress] [Amount]```
-CrabPing is the name of the cli
-HttpAdress is the address you want to ping
-Amount is how many pings you want to send, current limit is 200.

```bash CrabPing [HttpAdress]```
-CrabPing is the name of the cli
-HttpAdress is the adress you want to ping
