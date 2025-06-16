---
id: abstractions
aliases:
    - abstractions
tags: []
---

# abstractions

If you find yourself writing the same code over and over again, you can use an
abstraction.

For example, this:

```nix
{
  services.httpd.virtualHosts =
  { "blog.example.org" = {
      documentRoot = "/webroot/blog.example.org";
      adminAddr = "alice@example.org";
      forceSSL = true;
      enableACME = true;
  };
  "wiki.example.org" = {
    documentRoot = "/webroot/wiki.example.org";
    adminAddr = "alice@example.org";
    forceSSL = true;
    enableACME = true;
    };
  };
}
```

defines two virtual hosts with nearly identical configuration. To prevent this
duplication, we can use `let`:

```nix
let
  commonConfig =
  { adminAddr = "alice@example.org";
    forceSSL = true;
    enableACME = true;
  };
in
  {
    services.httpd.virtualHosts =
  { "blog.example.org" = (commonConfig // { documentRoot =
    "/webroot/blog.example.org"; });
    "wiki.example.org" = (commonConfig // { documentRoot =
    "/webroot/wiki.example.org"; });
    };
}
```

The `let commonConfig = ...` defines a variable named `commonConfig`.

The `//` operator is used to merge two attribute sets, so the configuration of
the second virtual host is the set `commonConfig` extended with the document
root option.

You can write a let wherever an expression is allowed. Thus, you also could
have written:

```nix
{
  services.httpd.virtualHosts =
    let commonConfig = { /* ... */ }; in
  { "blog.example.org" = (commonConfig // { /* ... */ });
    "wiki.example.org" = (commonConfig // { /* ... */ });
  };
}
```

but not `{ let commonConfig = ...; in ...; }` since attributes (as opposed to
attribute values) are not expressions.

**Functions** provide another method of abstraction.

For instance, suppose that we want to generate lots of different virtual hosts,
all with identical configuration except for the document root. This can be done
as follows:

```nix
{
  services.httpd.virtualHosts =
    let
      makeVirtualHost = webroot:
        { documentRoot = webroot;
          adminAddr = "alice@example.org";
          forceSSL = true;
          enableACME = true;
        };
    in
      { "example.org" = (makeVirtualHost "/webroot/example.org");
        "example.com" = (makeVirtualHost "/webroot/example.com");
        "example.gov" = (makeVirtualHost "/webroot/example.gov");
        "example.nl" = (makeVirtualHost "/webroot/example.nl");
      };
}
```

Here, `makeVirtualHost` is a function that takes a single argument `webroot` and
returns the configuration for a virtual host. That function is then called for
several names to produce the list of virtual host configurations.

[[modularity.md]]
