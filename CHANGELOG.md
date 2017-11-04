### 0.3.0

* Removed case_sensitive option from find function.
    Applying case insensitivity during the find/matching process is
    inefficient, especially if the collection being searched is going to
    live beyond a single search. Consumers are much better off mapping the
    values to lowercase on their own.
* Renamed Result type to Match, to prevent confusion with the std lib equivalent.
* Replaced find method ToString trait bound with a custom AsStr equivalent.
    This allows haystack types to avoid String allocations. Also added default
    implementations for common string types.
* Updated find function to return haystack references instead of cloned values.
    Incidentally, this removes the haystack type's Clone trait bound.
* Updated find method to accept iterator-based haystack.

### 0.2.0

* Added case_sensitive option to find function.

### 0.1.2

* Removed zero-score matches.

### 0.1.1

* Updated find function to borrow set rather than consuming it.

### 0.1.0

* Initial release.
