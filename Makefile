all: Test.pdf Test.html

Test.pdf: Test.md Test.css
	pandoc -s -f gfm -o Test.pdf --pdf-engine=wkhtmltopdf -c Test.css -V L=0mm -V R=0mm -V T=0mm -V B=0mm -V page-size=A4 --pdf-engine-opt="--disable-smart-shrinking" Test.md
	
Test.html: Test.md Test.css
	pandoc -s -f gfm -o Test.html -c Test.css Test.md

clean:
	rm Test.pdf Test.html
