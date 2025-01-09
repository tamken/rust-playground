# 1.83 / 241220
FROM rust:bullseye

RUN apt-get update && apt-get install -y \
	curl \
	git \
	locales \
	vim \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/* \
	&& sed -i -E 's/# (ja_JP.UTF-8)/\1/' /etc/locale.gen \
	&& locale-gen \
	# add rustfmt(formatter), clippy(linter)
	&& rustup component add rustfmt clippy \
	# add cargo-make, sea-orm-cli
	&& cargo install --force cargo-make sea-orm-cli

ENV LANG ja_JP.UTF-8
