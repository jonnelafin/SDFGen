(function() {var implementors = {};
implementors["byteorder"] = [{"text":"impl PartialOrd&lt;BigEndian&gt; for BigEndian","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;LittleEndian&gt; for LittleEndian","synthetic":false,"types":[]}];
implementors["chrono"] = [{"text":"impl PartialOrd&lt;NaiveDate&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;NaiveDateTime&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;IsoWeek&gt; for IsoWeek","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;NaiveTime&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; PartialOrd&lt;Date&lt;Tz&gt;&gt; for Date&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone, Tz2:&nbsp;TimeZone&gt; PartialOrd&lt;DateTime&lt;Tz2&gt;&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;'g, T:&nbsp;?Sized + Pointable&gt; PartialOrd&lt;Shared&lt;'g, T&gt;&gt; for Shared&lt;'g, T&gt;","synthetic":false,"types":[]}];
implementors["deflate"] = [{"text":"impl PartialOrd&lt;Compression&gt; for Compression","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;MatchingType&gt; for MatchingType","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L:&nbsp;PartialOrd, R:&nbsp;PartialOrd&gt; PartialOrd&lt;Either&lt;L, R&gt;&gt; for Either&lt;L, R&gt;","synthetic":false,"types":[]}];
implementors["image"] = [{"text":"impl PartialOrd&lt;NormalForm&gt; for NormalForm","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Delay&gt; for Delay","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; PartialOrd&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["png"] = [{"text":"impl PartialOrd&lt;Transformations&gt; for Transformations","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl PartialOrd&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Timespec&gt; for Timespec","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;SteadyTime&gt; for SteadyTime","synthetic":false,"types":[]},{"text":"impl PartialOrd&lt;Tm&gt; for Tm","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()