.POSIX:

default: build

run:
	hugo server --buildDrafts

build:
	hugo --minify

new:
	hugo new --kind default posts/$(name)/index.md
	${EDITOR} content/posts/$(name)/index.md
