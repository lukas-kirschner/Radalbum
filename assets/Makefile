all: Album.pdf Album.html

Album.pdf: Album.md Album.css
	pandoc -s -f gfm -o Album.pdf --pdf-engine=wkhtmltopdf -c Album.css -V L=0mm -V R=0mm -V T=0mm -V B=0mm -V page-size=A4 --pdf-engine-opt="--disable-smart-shrinking" Album.md

Album.html: Album.md Album.css
	pandoc -s -f gfm -o Album.html -c Album.css Album.md

clean:
	rm Album.pdf Album.html
