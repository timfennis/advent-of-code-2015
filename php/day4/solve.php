<?php

$i = 0;

while (true) {
	if (str_starts_with(md5("iwrupvqb$i"), "000000")) {
		echo $i; die;
	}
	$i++;
}
