{ fetchbower, buildEnv }:
buildEnv { name = "bower-env"; ignoreCollisions = true; paths = [
  (fetchbower "foundation-sites" "6.2.1" "*" "0lvg08sblpa7lzhdi7dy3y884fnw386aizb585gc6lyg25drhg84")
  (fetchbower "font-awesome" "4.6.1" "*" "00v785l3fj0sq0z3ca7ma2fr188x943s27rqpjjp6v4763j7jpp9")
  (fetchbower "jquery" "2.2.3" "~2.2.0" "1b4g42vmv8w5z1mnlbmw3cnvfkxaq957y42v9dvhwph1cqbrr85b")
  (fetchbower "what-input" "2.0.1" "~2.0.0" "0dlhmb4gifcap27cpdari4pic96znbypk6jcf2syq7ca446mqx3n")
]; }
