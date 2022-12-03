/*
This file is auto-generated from the public API of the zstd library.
It is released under the same BSD license.

BSD License

For Zstandard software

Copyright (c) 2016-present, Facebook, Inc. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

 * Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

 * Neither the name Facebook nor the names of its contributors may be used to
   endorse or promote products derived from this software without specific
   prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* automatically generated by rust-bindgen 0.63.0 */

extern "C" {
    #[doc = " ZDICT_trainFromBuffer():\n  Train a dictionary from an array of samples.\n  Redirect towards ZDICT_optimizeTrainFromBuffer_fastCover() single-threaded, with d=8, steps=4,\n  f=20, and accel=1.\n  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,\n  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.\n  The resulting dictionary will be saved into `dictBuffer`.\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n  Note:  Dictionary training will fail if there are not enough samples to construct a\n         dictionary, or if most of the samples are too small (< 8 bytes being the lower limit).\n         If dictionary training fails, you should use zstd without a dictionary, as the dictionary\n         would've been ineffective anyways. If you believe your samples would benefit from a dictionary\n         please open an issue with details, and we can look into it.\n  Note: ZDICT_trainFromBuffer()'s memory usage is about 6 MB.\n  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.\n        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.\n        In general, it's recommended to provide a few thousands samples, though this can vary a lot.\n        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_params_t {
    pub compressionLevel: libc::c_int,
    pub notificationLevel: libc::c_uint,
    pub dictID: libc::c_uint,
}
extern "C" {
    #[doc = " ZDICT_finalizeDictionary():\n Given a custom content as a basis for dictionary, and a set of samples,\n finalize dictionary by adding headers and statistics according to the zstd\n dictionary format.\n\n Samples must be stored concatenated in a flat buffer `samplesBuffer`,\n supplied with an array of sizes `samplesSizes`, providing the size of each\n sample in order. The samples are used to construct the statistics, so they\n should be representative of what you will compress with this dictionary.\n\n The compression level can be set in `parameters`. You should pass the\n compression level you expect to use in production. The statistics for each\n compression level differ, so tuning the dictionary for the compression level\n can help quite a bit.\n\n You can set an explicit dictionary ID in `parameters`, or allow us to pick\n a random dictionary ID for you, but we can't guarantee no collisions.\n\n The dstDictBuffer and the dictContent may overlap, and the content will be\n appended to the end of the header. If the header + the content doesn't fit in\n maxDictSize the beginning of the content is truncated to make room, since it\n is presumed that the most profitable content is at the end of the dictionary,\n since that is the cheapest to reference.\n\n `maxDictSize` must be >= max(dictContentSize, ZSTD_DICTSIZE_MIN).\n\n @return: size of dictionary stored into `dstDictBuffer` (<= `maxDictSize`),\n          or an error code, which can be tested by ZDICT_isError().\n Note: ZDICT_finalizeDictionary() will push notifications into stderr if\n       instructed to, using notificationLevel>0.\n NOTE: This function currently may fail in several edge cases including:\n         * Not enough samples\n         * Samples are uncompressible\n         * Samples are all exactly the same"]
    pub fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut libc::c_void,
        maxDictSize: usize,
        dictContent: *const libc::c_void,
        dictContentSize: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: ZDICT_params_t,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_getDictID(
        dictBuffer: *const libc::c_void,
        dictSize: usize,
    ) -> libc::c_uint;
}
extern "C" {
    pub fn ZDICT_getDictHeaderSize(
        dictBuffer: *const libc::c_void,
        dictSize: usize,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_isError(errorCode: usize) -> libc::c_uint;
}
extern "C" {
    pub fn ZDICT_getErrorName(errorCode: usize) -> *const libc::c_char;
}
