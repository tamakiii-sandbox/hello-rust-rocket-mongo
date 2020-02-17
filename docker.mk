
install: \
	.env \
	build

.env:
	touch $@
	echo "ENVIRONMENT=production-pseudo" >> $@

build:
	docker-compose build

clean:
	rm .env
