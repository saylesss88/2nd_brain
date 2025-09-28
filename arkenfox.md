## Arkenfox

Prefs are settings that control Firefox's behavior. Some can be set from ☰
Settings and all can be found in `about:config`, except for what are called
hidden preferences which will only show when they are modified (i.e. set to any
value by the user or browser - they have a trash can symbol for resetting)

### user.js

A `user.js` file is a javascript file and is text based, and resides in your
profile folder. It is used to set preferences for that profile when Firefox
starts. You can update the `user.js` while Firefox is open, it is only ever read
when Firefox starts.

Prefs must follow Mozilla's syntax which is `user_pref("prefname", "value");`

- the pref name must be wrapped in quotes

- string values must be wrapped in quotes

- prefs are case sensitive

- a semi-colon is required at the end

```js
user_pref("pref.name.string", "i am a string"); // strings require quote marks
user_pref("pref.name.boolean", true);
user_pref("pref.name.integer", 0);

user_pref("pref.name", "value"); // this will NOT be applied, it is missing a closing ;
user_pref("pref.name.integer", "0"); // this will be applied but NOT do anything (type mismatch)

user_pref("browser.startup.page", 0); // this will be applied and actually work
user_pref("browser.Startup.page", 0); // this will be applied but NOT do anything (incorrect case)

/* this is a
   multi-line comment
   user_pref("preference.name", "value")
   and nothing in it is applied ***/

// two forward slashes indicate a comment
// and do not need to be closed at the end
// and only apply to the one line from when they are added
user_pref("pref.name.example", "value"); // comment starts at //

user_pref("pref.name.example", "not commented out"); // this is an ACTIVE pref and will be applied
// user_pref("pref.name.example", "commented out"); // this is an INACTIVE pref and will NOT be applied
```

### Usage

- Firefox starts
  - It reads the ACTIVE prefs in `user.js`, in order, and writes them to
    `prefs.js`
  - It ignores INACTIVE prefs, i.e., it doesn't reset them
  - If a pref already exists in `prefs.js`, it overwrites it
  - If a pref is listed twice, the last one will be applied
  -

## Check Browser Console

`Ctrl-Shift-J`
