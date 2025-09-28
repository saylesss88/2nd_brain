## I2P

`.onion` is a hidden service

I2P has their own version of hidden services that end in `.i2p`

<https://geti2p.net/en/>

With I2P is more of a protocol, everyone using it is a router.

peer to peer network that can be

`pkgs.i2p`, `pkgs.i2pd-tools`

## Firefox Configuration

In Firefox go to Network Settings in the General Tab

Go to Manual proxy configuration

HTTP Proxy 127.0.0.1

[x] Also use this proxy for FTP and HTTPS

[x] SOCKS v5

No proxy for:

```text
localhost,127.0.0.1
```

---

Once this is configured you can only go to `.i2p` sites, it's recommended to use
a separate profile for this.

Go to

```url
127.0.0.1/7657/config
```

Check your network status, if its OK or Firewalled you're ok.

It's a peer-to-peer network, it takes some time to connect to certain things
once you first set it up. When you first set it up there won't be many sites
available to you.

## How to find eep sites

eepstatus

```url
identiguy.i2p
```

It's best to not use Windows with this, use a free open source OS like Linux.

### Jump Service

When you go to an eep site, sometimes it will direct you to a Website Not Found
in Addressbook page that lists jumper services.

Could not find the following destination:

<http://bible.i2p/>

Click a link below for an address helper from a jump service

<stats.i2p jump service>

He tried both and the following worked:

<notbob.i2p jump service>

It directed to a `Host: bible.i2p` where you can click the `B32 Link`

You can then save the address to your addressbook

## Search Engines

```url
legwork.i2p
```
