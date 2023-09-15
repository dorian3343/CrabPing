# A cli tool made for pinging http endpoints.

*This is a personal tool so don't expect excellent maintenence.
*Currently no support for payloads

## How to install 1/2 

1. copy the git repo 
```console
 git clone https://github.com/dorian3343/CrabPing
```
3. Build it with cargo build --release.

4. On linux, find the binary in target/release.
- Add it to your path 
```console
 sudo cp CrabPing /usr/local/bin/
```

4. On windows, idk I don't use windows :P 

## How to install 2/2
1. Instead of building the project yourself use the prebuilt binary

## How to use

```console
 CrabPing [HttpAdress] [Amount]
```
-CrabPing is the name of the cli \
-HttpAdress is the address you want to ping \
-Amount is how many pings you want to send, current limit is 200. 

```console
 CrabPing [HttpAdress]
``` 
-CrabPing is the name of the cli \
-HttpAdress is the adress you want to ping

## Example

```console
CrabPing https://dummyjson.com/test 5
```
