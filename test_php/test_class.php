<?php
Class Data {
	public static function dangerous($param) {
		$temp = json_encode($param);
		// this should not alert because the variable is not in scope
		printf($user_input);
		// test recursion
		dangerous($temp);
		// test return
		return $temp;
	}
	public $a = 0;
}



$user_input = (int) $_GET['input'];
$improperly_filtered = "$user_input";
$d = new Data;
$t = $d->dangerous($_GET);

// this does not alert because the input was sanitized
query($improperly_filtered);

// this does alert because magic quotes dont stop xss
echo $improperly_filtered;
printf($improperly_filtered);

// alerts because taint follows through method call into $t
query($t);

$x = (($t) >= 1) ? 1 : 2;

query($x);
?>
