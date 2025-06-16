---
id: GPG
aliases: []
tags: []
---

`GPG` is a popular Linux encrypring tool. 

The GNU Privacy Guard (GPG or gpg) tool is a native/baseos security tool for
encrypting files.
> gpg is the OpenPGP (Pretty Good Privacy) part of the GNU Privacy Guard (GnuPG)
> It's a tool to provide digital encryption and signing services using the
> OpenPGP standard. gpg features complete key management and all the bells and
> whistles you would expect from a full OpenPGP implementation

- Create or encrypt `(-c)`

- Decrypt `(-d)`

## Encrypting a file

```bash
$ echo This is an encryption test > file1.txt

$ gpg -c file1.txt

 lqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqk
 x Enter passphrase                                     x
 x                                                      x
 x                                                      x
 x Passphrase: ***********_____________________________ x
 x                                                      x
 x       <OK>                              <Cancel>     x
 mqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqj

 lqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqk
 x Please re-enter this passphrase                      x
 x                                                      x
 x Passphrase: ***********_____________________________ x
 x                                                      x
 x       <OK>                              <Cancel>     x
 mqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqj

$ ls

file1.txt file1.txt.gpg

```

Encrypting a file with gpg leaves the original file intact, file1.txt, and adds
the telltale .gpg extension to the newly encrypted file.

- You should probably remove the original file, so that the encrypted one is the
  sole source of the info contained in it.

- Alternatively, if you're going to share the encrypted version, you can rename
  it before sharing.

- The `.gpg` extension isn't required, but it does let the user know which
  decryption tool to use to read the file. You can rename the file anything you
  want.

  ```bash
  $ file file2.txt.gpg
  file2.txt.gpg: GPG symmetrically encrypted data (AES cipher)

  $ mv file2.txt.gpg testfile01.doc

  $ file testfile01.doc
  testfile01.doc: GPG symmetrically encrypted data (AES cipher)
  ```

## Decrypting a file

Decrypting a file means that you remove the encryption to read the file's
contents. There's no extraction of content or creation of the original file
when you decrypt.

```bash
$ cat cfile.txt

This is an encryption and decryption test

$ gpg -c cfile.txt

< Set passphrase and repeat passphrase >

$ ls

$ cfile.txt cfile.txt.gpg

$ rm cfile.txt

$ gpg -d cfile.txt.gpg 
gpg: AES encrypted data
gpg: encrypted with 1 passphrase
This is an encryption and decryption test

$ ls
cfile.txt.gpg

$ cat cfile.txt.gpg 
o@yAw?D??^a??!s?????;??!?v9-3, ???XA??!?9v?}???
Ž??m??1./fKˡ??R???:j?F?|?AS?O
```

Note that there was no passphrase prompt to decrypt the file. If you want to be
prompted to enter the password to decrypt the file again, you'll have to wait
ten minutes, which is the default timeout value.

## Decrypting and extracting a file

If you want to extract the original file while decrypting it, strangely enough,
you issue the gpg command with no options.

```bash
$ ls
cfile.txt.gpg
$ gpg cfile.txt.gpg 
< Passphrase prompt >
gpg: WARNING: no command supplied.  Trying to guess what you mean ...
gpg: AES encrypted data
gpg: encrypted with 1 passphrase
$ ls
cfile.txt  cfile.txt.gpg
```
