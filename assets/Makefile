all: Album.pdf Album.html

Album.pdf: Album.md Album.css
	pandoc -s -f gfm -o Album.pdf --pdf-engine=wkhtmltopdf -c Album.css -V margin-top=0 -V margin-left=0 -V margin-right=0 -V margin-bottom=0 --pdf-engine-opt="--page-width" --pdf-engine-opt="210" --pdf-engine-opt="--page-height" --pdf-engine-opt="1189" -V dpi=300 --pdf-engine-opt="--disable-smart-shrinking" --pdf-engine-opt="--print-media-type" Album.md

Album.html: Album.md Album.css
	pandoc -s -f gfm -o Album.html -c Album.css Album.md

clean:
	rm Album.pdf Album.html
