# mem TODO

* [x] Move from Option<T> to Result<T, E>
  - Custom error type? Sadface.
  - Could just do a string as the error, I guess.

* [ ] Batch API?
  - Batch read X continous addresses. Would allow more efficient reads.
  - Not that relevant for memory, but for disks, RPC and anything with DMA.