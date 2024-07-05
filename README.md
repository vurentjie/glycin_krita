> Note: Gnome's new default image viewer is called [Loupe](https://welcome.gnome.org/app/Loupe/). 
>
> If you are using the other image viewer called [Eye of Gnome](https://wiki.gnome.org/Apps/EyeOfGnome) you can install loaders for that from the [kra-gdk-pixbuf repository here](https://github.com/vurentjie/kra-gdk-pixbuf).


# Krita glycin image loader for Loupe image viewer. 

Adds support for `image/openraster (.ora)` and `application/x-krita (.kra)` files.

This assumes you have installed the [Loupe Image Viewer](https://apps.gnome.org/Loupe/) from [FlatHub](https://flathub.org/apps/org.gnome.Loupe).

> Please note if you uninstall or re-install Loupe Image Viewer, then you will need to copy these files again. 

## Installing from release download

Download `glycin_krita_<VERSION>_amd64.zip` from the [releases page](https://github.com/vurentjie/glycin_krita/releases)  and extract the contents.    

There are two files included:
- **glycin-krita**  (the loader binary)
- **glycin-krita.conf** (the loader config file)

Copy these to the following locations. 

```
<loupe_install_location>/files/libexec/glycin-loaders/1+/glycin-krita
<loupe_install_location>/files/share/glycin-loaders/1+/conf.d/glycin-krita.conf
```       

`<loupe_install_location>` is the path returned from `flatpak info --show-location org.gnome.Loupe` .

You will need to restart Loupe after that.

Demo:

https://github.com/vurentjie/glycin_krita/assets/639806/71cb26d7-6414-4104-aba3-3de2cde4ab3d


## Building

If you are familiar with building rust applications run the following commands.
Depending on if you installed the application per-user or system-wide, you will
need to choose where to copy the loader files.

```bash
git clone https://github.com/vurentjie/glycin_krita
cd glycin_krita
cargo build --release

#per-user install
LOUPE_INSTALL_PATH=$(flatpak info --user --show-location org.gnome.Loupe)
[[ -d "${LOUPE_INSTALL_PATH}" ]] && cp ./target/release/glycin-krita ${LOUPE_INSTALL_PATH}/files/libexec/glycin-loaders/1+/glycin-krita
[[ -d "${LOUPE_INSTALL_PATH}" ]] && cp ./glycin-krita.conf ${LOUPE_INSTALL_PATH}/files/share/glycin-loaders/1+/conf.d/glycin-krita.conf

#system-wide install
LOUPE_INSTALL_PATH=$(flatpak info --system --show-location org.gnome.Loupe)
[[ -d "${LOUPE_INSTALL_PATH}" ]] && sudo cp ./target/release/glycin-krita ${LOUPE_INSTALL_PATH}/files/libexec/glycin-loaders/1+/glycin-krita
[[ -d "${LOUPE_INSTALL_PATH}" ]] && sudo cp ./glycin-krita.conf ${LOUPE_INSTALL_PATH}/files/share/glycin-loaders/1+/conf.d/glycin-krita.conf

```


## Additional Notes
This mostly a stripped down version of the base loader in the 1.0.1 tag of glycin source repository
[found here](https://gitlab.gnome.org/sophie-h/glycin/-/blob/1.0.1/loaders/glycin-image-rs/src/bin/glycin-image-rs.rs?ref_type=tags).
I haved kept parts related to loading png images and added support to extract "mergedimage.png" from the .kra or .ora file.

