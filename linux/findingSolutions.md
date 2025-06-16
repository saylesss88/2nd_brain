---
id: findingSolutions
aliases:
  - Tips to improve your search results
tags: []
---

# Tips to improve your search results

As Garuda tracks very closely with upstream Arch, most information (but not all)
contained in the Arch Wiki is likely applicable to Garuda.

You can use Arch Wiki's internal search engine, but it's prob more convenient to
just use your favorite search engine.

Running an internet search on a topic such as, "Archwiki Pacman usage" will likely
give you a lot of relevant results.

- Man pages are a good place to start. `pacman -R --help` or `man pacman`

- Most search engines have a feature that allows you to limit search results to
  the last week or month.

- A bug that appeared with your last update is more likely to be found in very
 recent online posts. Learning how to do this is is the first step in becoming a
 search guru.

- You can refine your search even further by forcing the search engine to
include results for a specific word, or several that are the most important to
your search. You can do this by surrounding the search term with quotes.

- One of the most effective methods is to use "Arch Linux" as your first search
  string delimiter. This will help minimize extraneous hits from Ubuntu/Mint etc
  forum threads.

- To narrow your search results even further use alternate distros such as
"EndeavorOS" or "Manjaro Linux" or "Garuda Linux".

-  If you can't readily find your targeted search term listed on a very
lengthy webpage then use the "Find in this page" feature included in Firefox
(or other browsers) to locate the specific info quickly.

Further ways to fine tune search methods

Below are some further ways to fine tune your search methods so that the hits returned are far more closely related to the specific answer you are seeking. This is an example of a problem that occurred whenever a computer was restarted, suspended, or shutdown. A loud popping sound through the computers speakers was happening whenever a power cycling event took place. Running a search that included the triggers “start, suspend, shutdown” returned mostly posts that were unrelated to the issue at hand. This will happen when you include too many general search terms, and you will get a million disjointed jumbled hits mostly unrelated to your problem returned. The key in this type of situation is to search each issue separately, rather than searching all the issues together to get a far more focused result.

    Example, run a search for "Arch Linux loud sound at shutdown", with "Arch Linux" in quotation marks. At first, run each search limiting the time frame to the last year. This is a very useful way to start off your internet search. As generally, fairly recent posts tend to be far more relevant today than a hit from a decade ago.
    Then try "Arch Linux loud popping sound at startup", with "Arch Linux" in quotation marks. Again limit the search results to the last year. If however, at any point your search results start to return very few relevant hits, then remove the time restrictions.
    Then search "Arch Linux loud noise when suspending". Again limit the time frame to the last year.  Subtly change your search terms after each run to hopefully improve the accuracy of your results. If you've run all those searches using “Arch Linux” as a delimiter without success, then rerun your searches using distros that are Arch derivatives. If again you have no success, then lastly perform the searches again, this time using “Ubuntu” instead of restricting the search to Arch (or Arch derivatives). Hopefully by the time you've run all these search permutations you've found pertinent answers to some or all of the issues that were occurring.

Here are some further advanced tips on how to refine your search parameters. I realize some people justifiably refuse to use Google, but as it is the most popular search engine in use the following tips will be Google specific. These tips may, or may not work with other internet search engines. 

You can narrowly focus your search to only one specific web site if you want to avoid searching the entire web. This can also be done by domain, but I won't get into that. 

Often the Arch Linux forum is the best place to begin a search. The Arch Linux forum web address is: 

https://bbs.archlinux.org/ 

Say you wanted to search only the Arch Linux forum for results relating to widgets, you would preface your search with "site:", as so: 

site:bbs.archlinux.org widget 

Now say you want to omit weather widgets from the search results. You can have those results ommited by prefacing that term with a minus symbol, as so: 

site:bbs.archlinux.org widget -weather 

You can further refine your search results to include or exclude more search terms. You can make your search more specific by adding multiple search terms with a space between each, as so: 

site:bbs.archlinux.org widget kde 

The above search terms will provide results from the Arch Linux forum focusing on KDE widgets. 

You can also add further terms to exclude from search results. Weather and network  widgets can be excluded from the overall KDE widget results on the Arch forum, as so: 

site:bbs.archlinux.org widget kde -weather -network 

When you combine all these methods together you can get far more refined search results than you ever imagined was possible. Hopefully these extra tips will give you even more options to vastly improve your search results.

    Below are even further tips to improve your search success rate. Ninety five percent of solving most Linux problems is about learning good search techniques to quickly find the answer you need. The most important thing is to learn to search methodically. You can only be sure of one thing, there's no way that you're the only one on the entire planet experiencing your issue. You simply need to refine your search skills so that you can locate relevant information pertaining to your issue at hand. Generally, if you search methodically and thoroughly you will almost always find information online about the issue you are experiencing. Usually, if you search hard enough solutions or workarounds have often already been found, (but not always). In these stubborn cases filing a bug report with the upstream project may be the only way to get a resolution.
    Luckily, in most cases it simply takes patience, perseverance, and good note keeping to solve the average Linux problem. Always save any promising leads you find online as webpage complete in HTML format in a directory specifically dedicated to your problem. Firefox has an excellent extension to save a complete webpage in one HTML file, the extension is called "SingleFile". Saving your finds as webpage complete in HTML format will ensure your information is compatible with any web browser you use in the future.  Once you have tested any solution (or command), record the results immediately in your notes. Don't leave troubleshooting complex issues to your memory. Always record everything you've tried and the result in detailed organized notes for reference later. If you need to reinstall in a year you will want everything recorded, so you don't need to repeat your search for the same answer all over again.
    When answering help requests on any Linux forum it never ceases to amaze me how many users will post a rather long and seemingly unique error message, but when you ask them what the search results of the error turned up most have never even searched the error message. Error messages contained in dmesg or journalctl logs are your best friend for solving complex Linux problems. Learn how to search your logs for error messages, and then research any error messages you've uncovered online for others with similar errors. This is one of the surest methods to track down what is causing any problem.
    There are times when searching the complete contents of an error message will produce severely limited (or no) search results. Oftentimes error messages contain long alphanumeric strings that relate to the specific hardware in use. If you search an error message and get no useful search hits, try progressively removing the alphanumeric strings from your search terms to hopefully improve your results.
    Below is an example of an error message containing several alphanumeric strings:

amd_gpio AMDI0030:00: Failed to translate GPIO pin 0x003D to IRQ, err -517

 

    When the entire error message above is searched there will likely be zero hits returned. If you instead search the error message with the alphanumeric strings removed (as below) you should now receive useful search hits.

amd_gpio Failed to translate GPIO pin to IRQ, err -517

 

    Searching the streamlined error message with the counterproductive strings removed returns over 1500 results, while the original error message had no useful search hits. This example clearly demonstrates how a minor change in search terms can dramatically affect your search results. Learning to tweak your search terms is an acquired skill, and the more you practice, the better your search results should get.
    Learning how to search effectively, and keeping detailed notes on all your efforts is the key to finding the solution to most any complex Linux problem. I'll let you in on a another secret, most of the forum support volunteers don't have the answer to every support issue on the tips of their tongues. Experienced forum troubleshooters have simply honed their search skills from years of repetitive online searches. This often enables us to find answers far easier and quicker than the average user. However, there is absolutely no reason why the average user can't become as equally effective at researching and solving their own issues.
    This distribution is all about trying to teach new users how to do for themselves. We have found that spoon feeding new users a copy and paste command line solution accomplishes very little for the user, (or the distro). This is because the very same user will inevitably be back shortly thereafter with another string of elementary questions, (and they will again be expecting to be spoon fed the answers). This unfortunately teaches the user little about how Linux works, or how to cope with issues on their own. We at Garuda try our best to break this dependency cycle by attempting to motivate our users to expand their Linux capabilities by becoming more self reliant.
    Some Linux users don't subscribe to this philosophy and think it is rude or elitist to expect new users to research their own issues themselves. Those involved on the Garuda project disagree, as we believe instilling a desire to learn for one's self is more important than an effortless fix. Our philosophy is to promote capable, confident, self reliant Linux users, who in the end don't need to go fishing on the forum for answers anymore. We hope the new users asking questions when they first join our forum, will be the ones helping out other users here in a year's time. This is our goal at Garuda, and hopefully you can understand why this is beneficial for everyone using the distro in the long run.
    Good luck finding your answer! ✌️


