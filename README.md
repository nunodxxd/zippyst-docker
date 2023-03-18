# Unofficial Docker Image for Zippyst

Extract direct download link from a [zippyshare.com](https://www.zippyshare.com) page without rust stuff.

This is an unofficial repository that aims to provide a Docker image for zippyst.

[Official Repo & Docs](https://github.com/scotow/zippyst)

# Usage
You can pull the image from Docker Hub using the following command:
```sh
docker pull nunodxxd/zippyst
```
Extract one link:
```sh
docker run -ti --rm nunodxxd/zippyst 'https://www3.zippyshare.com/v/CDCi2wVT/file.html'
```
Extract multiple links:
```sh
docker run -ti --rm nunodxxd/zippyst 'https://www3.zippyshare.com/v/CDCi2wVT/file.html' 'https://www3.zippyshare.com/v/CDCi2wVT/file.html' 'https://www3.zippyshare.com/v/CDCi2wVT/file.html'
```

## Extract multiple links with file.txt:
1 - First, you need to create a file eg:links.txt and add the list of links to it, one link per line without quotes, like this:
```sh
https://www51.zippyshare.com/v/9Dw67H0Y/file.html
https://www13.zippyshare.com/v/xPvlkk08/file.html
https://www52.zippyshare.com/v/To28kzAt/file.html
https://www51.zippyshare.com/v/Gnxjl25S/file.html
```
2 - Next, you run the following command in the terminal where you create the file:
```sh
docker run -ti --rm nunodxxd/zippyst $(cat links.txt)
```

### Examples

The links bellow were used to demonstrate the usage of the command. They may have expired.


### Disclaimer

This repository and the Docker image provided here are not associated with zippyst repo. The image may not be up to date and may contain bugs. Use it at your own risk.

# Bonus Pro-tip

If you're having difficulty obtaining the list of FileCrypt links, you can utilize the [Bypass FileCrypt](https://greasyfork.org/en/scripts/403170-bypass-filecrypt).
