<?php

function test($p1, $p2) {
	query($p1);
	$local_var = query($p2);
	return $local_var;
}

?>
