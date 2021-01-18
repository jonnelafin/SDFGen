(function() {var implementors = {};
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T:&nbsp;?Sized + Pointable&gt; Deref for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;T&gt; Deref for CachePadded&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, '_&gt; Deref for ShardedLockReadGuard&lt;'_, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;?Sized, '_&gt; Deref for ShardedLockWriteGuard&lt;'_, T&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Deref for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Deref,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Deref&lt;Target = L::Target&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["image"] = [{"text":"impl&lt;P, Container&gt; Deref for ImageBuffer&lt;P, Container&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Pixel + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;P::Subpixel: 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Container: Deref&lt;Target = [P::Subpixel]&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["scopeguard"] = [{"text":"impl&lt;T, F, S&gt; Deref for ScopeGuard&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnOnce(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Strategy,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()