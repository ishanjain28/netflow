# Netflow ZMQ


This project is meant to do the following.

1. Collect Netflow V9 traffic
2. Re-encode it into TLV or JSON and send it over ZMQ to `ntopng`.

I'll go with TLV or JSON depending on how easy or difficult it is it encode it to TLV format.


# Limitations

1. Option flowsets are not supported.
2. Only Netflow V9 is supported.
