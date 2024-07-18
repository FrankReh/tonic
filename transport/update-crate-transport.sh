find src -type f -name '*.rs' -exec sed -i.bak 's/crate::transport::/crate::/g' {} \;

# afterwards, to remove the .bak files created, consider
#
# find src -type f -name '*.bak' -exec rm {} \;
