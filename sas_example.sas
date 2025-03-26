/* Necessary to specify where function packages are stored */
options cmplib=work.myfuncs;

/* Make the Rust function available to SAS */
proc proto package=work.myfuncs.uuid 
  label='Rust UUID v7 Generator';
  
  /* Update this path to your library location */
  link "/path/to/libsas_rs_uuidv7.so";
  
  char* uuidv7();
run;

/* Create a SAS wrapper function */
proc fcmp outlib=work.myfuncs.new_uuid;
  function new_uuidv7() $36;
    uuid = uuidv7();
    return(uuid);
  endsub;
run;

/* Generate 10 UUIDs using the fcmp function */
data test;
  do i = 1 to 10;
    length uuid $ 36;
    uuid = new_uuidv7();
    output;
  end;
run;

/* Display the results */
proc print data=test;
run;
