rm -rf ./docs
mkdir docs
cp -r target/doc/* docs/
echo "<meta http-equiv=\"refresh\" content=\"0; url=./sdfgen/index.html\" />" > docs/index.html
