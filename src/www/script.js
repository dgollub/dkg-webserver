'use strict';

(function info() {
  // old school JavaScript
  var $p = document.querySelector("#info");
  if (!$p) {
    $p = document.querySelector("body");
  }
  if (!$p) {
    document.writeln("<h1>Could not find #info element nor <body> tag.");
    return;
  }

  $p.innerHTML = "JavaScript works as well!";
})();
