# rsclp
Command line parser rust lib crate


Command line parser is able to parse process arguments.</br>
Arguments could be of two types, single character argument (i.g -c)
or long text argument (i.g. --config-file)</br>
#### Process argument could be of the following type:
 - Boolean: classic is -h to show process help.
 - Integer: an integer value for example verbosity level --verbose 5. 
 - Floating point: a floating point numer --ratio=123.25 . 
 - String: a text argument, classic configuration file path --config-file app.properties.

<p>Integer, Floating point and String option has a mandatory angument
while Boolean option does not require an argument.</p>



Argument can be passed in two ways:<br>        **--config-file=config/app.properties** or **--config-file config/app.properties**<br>the same example is valid for single character options **-c=config/app.properties** or **-c config/app.properties**.


Single character option can be pass grouped together (i.g -xvz).
**Be careful** if a single character argument needs an argument you have to pass it or 
adding **=** and the value **(i.g. -xvzf=file_to_compress.tar.gz)** or as next process argument
**(i.g -xvzf file_to_compress.tar.gz)**.
**NOTE:** No more than a single character option with a mandatory argument can be grouped.

**[documentation](https://docs.rs/rsclp/0.1.0/rsclp/)**

## New Features and bugs resolution

2024-March-03 - Bug description: mandatory options are checked even if help option or version is set.
> **Solution**: method check_mandatory_options is not private anymore and the user can callit after the help or version option check 