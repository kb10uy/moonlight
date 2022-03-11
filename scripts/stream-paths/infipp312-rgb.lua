local infipp = require("scripts.infipp");
local codec = require("scripts.codec");

local encode = infipp.encode_with(12, 3);
local decode = infipp.decode_with(12, 3);

return {
    encode = encode,
    decode = decode,
    filter = codec.rgb444,
};
