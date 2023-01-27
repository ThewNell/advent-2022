use core::hash::Hash;
use std::collections::HashSet;

pub fn part_one(input: &str) -> u32 {
    let sum = input
        .lines()
        .map(|l| {
            let line_len = l.len();
            let first_group = l
                .chars()
                .into_iter()
                .take(line_len / 2)
                .collect::<Vec<char>>();
            let last_group = l
                .chars()
                .into_iter()
                .skip(line_len / 2)
                .collect::<Vec<char>>();
            let intersections = intersection(vec![first_group, last_group]);
            let first_intersect = intersections.first().unwrap().to_owned();
            let priority = get_priority(first_intersect);

            priority
        })
        .sum::<u32>();

    sum
}

pub fn part_two(input: &str) -> u32 {
    let sum = input.lines().collect::<Vec<_>>();

    let total_chunks = (sum.len() + (3 >> 1)) / 3;
    let mut chunk = 0;
    let mut total = 0u32;

    while chunk < total_chunks {
        let groups = sum
            .iter()
            .map(|l| *l)
            .skip(chunk * 3)
            .take(3)
            .collect::<Vec<_>>();
        let first_elf = groups
            .get(0)
            .unwrap()
            .chars()
            .into_iter()
            .collect::<Vec<_>>();
        let second_elf = groups
            .get(1)
            .unwrap()
            .chars()
            .into_iter()
            .collect::<Vec<_>>();
        let third_elf = groups
            .get(2)
            .unwrap()
            .chars()
            .into_iter()
            .collect::<Vec<_>>();
        let intersect = intersection(vec![first_elf, second_elf, third_elf]);

        let intersection = intersect.first().unwrap().to_owned();
        total = total + get_priority(intersection);
        chunk = chunk + 1;
    }

    total
}

fn get_priority(symbol: char) -> u32 {
    let offset = if symbol.is_uppercase() { 38 } else { 96 };

    symbol as u32 - offset
}

fn intersection<R>(arrays: Vec<Vec<R>>) -> Vec<R>
where
    R: Clone + Copy + Eq + Hash,
{
    let mut intersections = arrays[0].clone();

    for buffer in arrays {
        let unique: HashSet<R> = buffer.into_iter().collect();
        intersections = unique
            .intersection(&intersections.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }

    intersections
}

#[cfg(test)]
mod day_3_tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let result = part_one(TEST_INPUT);

        assert_eq!(result, 157);
    }

    #[test]
    fn part_one_regression() {
        let result = part_one(PUZZLE_INPUT);

        assert_eq!(result, 7716);
    }

    #[test]
    fn part_two_works() {
        let result = part_two(TEST_INPUT);

        assert_eq!(result, 70);
    }

    #[test]
    fn part_two_regression() {
        let result = part_two(PUZZLE_INPUT);

        assert_eq!(result, 2973);
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

#[allow(dead_code)]
const PUZZLE_INPUT: &str = r#"ZTmtZvZLTFNLMQMNRvZncdcHwcScJvcdHnVfwV
zqjqrzqjCqrjtqWhqChqrznhcfdfVnVSVgcffcgJcSgd
rtDGpzjjqjlrGsWqBWFRsbTPFQMTbRNRFmbs
FFlnlnVlmQqcBVhBRrSrGSwVdRJbztSt
NPPNsffWfNztRZSJNG
WpgpTTHDpgTDDpMLPGgMHslmBmmHcBQnFmcqhmnjlqQm
VlVNLlPQhtnDRPnP
QgqTffzZqgvgzWmqqZmGcDthtRFvnnFnhJtJJDDt
WGTBzSqBQTQmZBWHswpNbswLbSNCNl
PFzFQDdLjMzzQDhnDNmwZmqwRsmRRmMVNM
GtSbbtlttvvtBvtHBmqqNqVVwsVgCmRw
brlvctHfrlrqvGvcTpjDdFnLDhdfjLhQpn
fTqzgFrcmzTrTNgMzFzTrgtbMtVMVPSLVbvSStRttPVV
lsHnlQhplQpsHhlDJsswNwZPZZVNwvnSZSVvVS
BJpsGWhWQQHdGHlpWpfgrmqgNBmTTBqzTrzr
PLPPrDNHDBnrWNBmjDmjbqqgzjgjQC
GJpFwpvFFsJpwvwwGwJZJRptzdQSSZTmCjTQmTSQjjZSggQQ
ffwGGtJGCfLcNMNNfNLW
nlTzllGwlQHHGMSrrhQLcvbcghgcvL
fFNpttBRFFRNtJJnfNWmqbhhZrZrcjgpvLrvrvcvbr
JBftFFsVRWtNWRsmWtRfftGHGPDDCnTDHSCwzwVnwGSw
LsrZLbLmPSTZtPcc
zpnpljpdwhRpfNLlPWPtTPcnMWPcctTV
qlhNNjldqdpgqdfhjpphdsgmmrDBBJJQgHrmDHLQJb
ndDrsBbpqspHCjzVBCHjMj
LWFcQFQtVVdcCMHC
RLthwTWLTSLwTllRLLlLNlPpPGfrGnqDdGqsZqsZsZhb
jHnJLJrcBbrRHJpnLBBjdHbjfgvGwstwWwdsWWshfhFzFfhv
PZDQMNTMQwtgvGTGhf
lqlPPQZQVSQCPSCPDmLppjqRqHngLHjRgLnL
MfQrhdzNMfMPsNNmPmqPLCLCVLBnbP
WVtcVvJJVjvFZZmgGjnjLBjmnnHB
FFtcZRpWJcSDTwlVMhMDwrrDzM
bSMbrQGZwwqbrbGMdTQGMwQdfFBLDLHLHssDgsHHBJDsFfwL
lpPPCccvVPvccPzcWLDJFJBsBzJJHsqHjg
tnqlPlVlNvNZZSSrGGZb
NlBBBBBwmwcMgLwLVVLLLscHdnMDdbHJCbdvJbPdDDJHvn
ZRWQWSfGhtSjqZhffZhhRjtdvPFHvCJbFdCFFDdHJdPb
fpqGhQWWZRhfqRSqrpGllLcVLmscgrLwBmgCzN
wHHHwDNCzZwzZfpzwsswzfzvnvnMDMgMhMGGFGRVdMnvcF
mLtttqTPBtSJTSBQWlmcMvVdclnvhFggdGhRdn
QBLPSLQQWmLtTBtBSjZfsbwbHzpjGzHfpzzw
zDPBqBPqwzzsqlTzzPzqjttNbCPJVtPCbrvjvCZP
MFhMhGfFfGpWWpHhFfRpHjVNbDbJjVMjrVvVNtMtbt
gfRSmpggSFRgfGRphFGgfgBSBTqzSsdlTLqdsnsDBqLw
QTTdWZZZTrhCZrjhVt
SBJJNRRvvRpwwspNSpTPjCMVhhtCjGThjBjB
fJJpvRRwNRSwvzRzRvzSgSRNqFTFDbFmqncQTbdFfnbDQnLW
wSjwwjWhbhhjWjdqSVpqSndvMdmM
NTvPBrZgPPHDFrFHGvZHRBqVBpCQVRRdmpRppdqM
TgDHZPsNTWbbwsjtvw
HVRPRwJVppQNpGmvMvmqqq
WcttbWsWsdDdbFDSSWclZGvFLMvfvjfzNvLGzLmfqv
tglcmSldWlBQBThRQBhg
LDGGfPFLGsfFnGLfMzfJVccNzmcmwm
qggSRgtjgCRtdtNZCJMCcZwTZVcm
RBbHbbRRnLBDDrvN
pThhMtMtTsWspWTnGjpsVHPdjdjvHdgVvLDVVSjd
JQNrrwCFrSRLRVLnSN
JQcJFCfcnfQmrbblQqlthGqzpZpGTBMWTMqtst
DzVmzDDgsNdHHLGJhJppPPllqSgbql
jrBWjcQMCRchPShwSblpbW
ZjRrSCtnnrtBQTcrSZmGtGFFNDHmHVVmVtzz
GSbGbrTGRRScMlVFjfbqqjdF
tvDmZhtNtHDttBhCmHDJHNwlpjslMwfflvpqldsFpMlf
HdBhLDhJDLWWLDCSQGcrQPGccLTGcQ
tNzrCdJBrrtQdtgQdlQQtrnMZhMTqzHqqTFZZFqTHFMV
bwbsRDvfwfsfcfSFqMmqDqFVNMNVVH
PRvfbSjNvPWPlLllLJ
MjMRRNRMjZtGVGpBCMMCDV
vwfhFzhvcJQQwJlQSddzQwFWGqcDpCDqBqDTcGZTTVTCWG
dvFJzrfZJhddLNLPHsrtRbPr
BmbsFNBhszGgGtGl
MSHVwdSwZflGlcqqpM
QvwZCnnnSvZdCSWrNmDlPPBPDhPmjW
MDwpRzRwMzMsdVSjdMWWMQdW
gmlftnDtHnHHVWSWBjWjWgvW
HbtrhbbGRNDpbCDz
cjVGqQqVqBFhDtvB
ZnTTfTffZZmDhBtJZG
fNzbpzlHTlgHNzlRTlRNbHrWwrCwWSSdQddNjwVcjGSd
WWrPMhwhnjpSLCpDlSSW
sNGBGdmTbZNGsmbstNLZGZDBRSlqSRCfRSCqRRDDqCqp
NvvcGtsLdTNZmLTmdJPcnnMFjFMcrVJhMF
wMRQMSQZHznRsqRbWp
ddgDDhfDrrDdjthHmdHrzpbVCVWFpfVqCWpWqpFs
hdvNdHtjMLJZvJwM
nWzZtWzHzZWgQHMNLDMDfDBfQbdD
MCPmRGGhqdmNjbDN
FRPlMlRChCvFggWZsHsZHn
ZGhhjdgwgcZHsPnRnSnbWscn
tltlfMQQQftDFJpMQJsWJWJsnWBnRSBbrBWP
lfLDpQMFMNfLjGjPPgZzwh
HcmvWcqnHLLPDzPPHM
MGGSfdJRdCglfrLjrjrLzLPzlF
CsgRgfwgdCwhsssJBBwvcnQnQNMvQtQm
nrVbwgnSTSgSnrZpjpWWqmWNHlqqpV
BcBPhSdsDlHNtNlJNP
RLFCQLQRRsBDDcRdGDddhCRDfwzvSCnzrbfrMfMgznwwfbZv
HJLzLNwBNzNJLzBJztRGzQVnDgwsjbgwssZwnDZbDQ
PhvlSvvPfMRlDbDsggbSjQSg
mFfhMlMrfchvPPpFTPvMvPzLBHWRHJNtzJGzzHpzBtCC
FLsgSLzLswdFgLBbWZnJDWHcmZnnBjHM
rpbrrqfqpvCbqqQQvvpClblRDHmjmmjJJZZnWMJmmJmmcRMc
qTppQvCfhzSVVTzbdL
ThTJtlqfDrDtffwqRCFCCnLwdnmpzRdF
WPQSSFsGMgQZWvFQgZgZQcCdmmzLMLCpNzCCddmLzdzm
cvVPGPQQGsZsgGPjVFccbHhThJHhDqfhDDjHqDrr
CgnCCnPMnMtGHDbMFQ
zLpwpRTwwRwhRchHwmqmGvvGbqmTmNQqvq
sHdhzlwrRVrdLzRrprrfjgfjVBnBCfSBZZgPfJ
SPMdWwWPrZwdrrWrSPLFDfgbQDfwDFfFglDQ
qLGBtGLpjzqmvQbvvpDvfFpR
jhLqqHBLGjtLqSZCssSTZZMshP
MhJCpPDpRRFRzQQNNqbcZjNZmVhjNm
LLSlLnGDmNqLbNjb
sfBngrBSTTSnSGHlTsHBGsBpFFdRdQPDCFPWRMzzdQWM
ZQtmgtWfPcgPgcsb
pvMhFThpHVTvPbcFcFJLJDsd
MTVHchVVHjHHTcpjMVBjjnQnGqQZnlqBmrWQQBffQQ
HfcRNJpJfhCmpGSqqGNjsjBnQl
tTdPwwtLTrrTVPSnQsbGPsnnlFllFQ
VvvTSTwWMrSZVwwrrmRmczJchHcHCZhRzD
wfRwhmLRnvrHqHhV
bJlBHlWlHMBPJzDlzMMJJSBlFnrGVrqspGGvpVGPVpGsVqvp
DWdStDMztCCHgZCtmZ
MHdsznVDDfcjcjDcdDjmMSCQwQpCpFCvqSZQqFzQpQ
lhJnLJJnTNrWTRqvqqPrFwZvwqQp
NghhtJBtnWLRTNjmcMDtjdsMfGDH
jgsvPffVmHfDqPSrNwnQwnwNhSvw
PZbGbCdcGdRCGtntQLQQLLwtpLNw
RbcPFBRFcdZBBJDBmTHjsD
dTTFJdzhmmmQpzVz
jtNLcctGGjtfrnVMsNQNQVVWnv
GcrcrfLtDggLftDFhZFdJFHJBVFBgZ
TTbqTgqCqZCrwmhQnnmrgh
MhpfsMLhfmrznLrQrF
RsStRMtjpHMfDtWsWsNDppsqlZqBlhNlbcNdTPClPqcvBP
lRhZPgnpRGZlSrmsLSvSzLVl
wwHdHCfDQCJHdwdDMdHCcDsLmNVvzVsWrcVNcVzLbrLz
dMCCwCtJdwDQJMtjhnvhpPhRZBhR
pBqMZfDffmBnvnNmPt
rhwLHCChrLPCMNWMCNmW
GSMVRSVwHLMRJDQJTZlJZR
sfstzPGRRBSngMfQLNNqgWLQLZZNgq
scVDjjjCDTVhHlDhHdvvjwjHrZmWQmJmrJJWqNqLJbrcQqZq
VvHlVHldTjvhpVplhVThhwjlPFSzPfGzpGBnsRffRRBPPGGF
WNFNfnWTSLSJTnWShTvVZCnvrdPrZvddVCrt
QwsMjppcpHCPdHsvPZ
lcwMGgpcGbzQpMgQwbDjDQZSJTRfShffWNJSSNFFbhSb
JDNgTgqDTggQbQGbZDWbJmVJrPVfPjlPfPwlljJC
FZzHFSznZZtptHzcSmCVrwfPVcwwVrCcdm
nMSStvnZFSHpLLtBtMzHnMWQvNhgDgGNRGNhgqRWTgqg
SJcrhvbBLBLrDpllvnwHQRnllHnQ
ffsjfMMZfVdCCgCfgTzmzslRtwFwFtTnqqHTJRQnqRqq
CVPgmdggVjCJSrhrbrPrrSLW
LPtcLtgddLMRRCMRpTBRrZnppvvGRvBw
WNNJjDjqSjJSqWqzNqzlSlBTGGFvrppSrwTFpn
qbNDWNNHbJqVtctwVmsfLCLP
FvSSLMqgvVSQjQfgwpwWpj
BthszRPRRNbNtzmHRbHNRNPfwJGcsswWQpffJpfsJcQFwJ
bBtzPmRrbBRHtNCzPhqdCLFMLSSvdnvCTnML
VPHWJPDjVLDDjDSFDJhgdnNGdbblzTzNjlnNbl
ZprsRZMQwwmGZsvtQZgTfggqnbfdTzrbqlTd
GZMtsscmsRZswwBQHBhDDJJPPCPWSWCJ
mNDNNmmVMSVgGgGGqsqGLhQqsLGhLq
nZBTZpJPhCpnnrsqbbcfczJfFccz
HpBZZRPRHjnPPjrHnRtCZnBdShDVlMDNDgVmtmDdVDWSdN
tBftztmztGBBCBSGHBmhvHHcchbshhThpbLJHJ
wwzMrrMnQdldVdMvJTcLNnphphbLJv
ZzwPVrWQlwrdStGGCWqDSSGW
QwfrQPvhwPfzQrvWWpQpvVGGTDGsjbgNNcbfsGTsDFgG
CtddSdZMRRdnJhRnHtZtlRMbGGDjDgggjNTZDNgTGFgGjc
mdmdCnHhVWmLmwwL
zLcWSWFcPJLWrWLSZrJLjVjHtjVsrdtstHdtVQgg
nChlwwnmhlCNqhhjHMgDjVVdwMjdtd
CNnBmNNThhhdhCdlBGGlGvNpcJbJLSbcZzcFJzpJTWPbzc
LdPZTPVpLCVTtCNsNsfFnlDC
SMwqcqcWQMbMhWQzBnsNfsFwrnnNNlrs
WMMWhvQRNNNjvLgZ
DWFGzrtfsZHZZMLt
pNwNzNCNTpppmnvNMTLVjHLBLLjMRTLH
PPdlPmJJNNClDdcdDDfWhzrW
nSJVSHQQnwLThnhrML
ddsjfRdGZjmGjRTwwTZhwrMwWwtb
qCdfRdMmgssPfjsdjdPspBzQpScSSCBpzNBQzcQz
fJnmRMJrlrmRmTRmbqssWVdqNVQdswdNNb
GZggFHGhHHgHSFvtHPPPsfwgwNsVqjqNpNjNNssN
PPSPDDBPBmBMlLfmLr
BdqdCBqqCVPVTZBrlJcTcTJTcfcbwwmcgv
WjGGLzLMhpWQmRGhpHfbhcDhHHHhgcsbJD
tQzSGjWRzWBntntrZmVB
clfLQLgfzfTLDMwNrNrrNDGCGG
tmbpFtBvvmvdQQdFQwMJCG
SnbtnqnSbnQQsBqzgLgVsLZTLTPfVg
QnQBQQBVzqqzpmfgBpnqSDFPjhhWsFVhlsFFsDstFs
MGGrTHcvRTTrrrCDpjvWtFPlFlsvjp
bZbpTpJJBBQmBmJf
dNVgDdVtPcNPhgTLPLpTPlnTHHRn
WrvjvwjWwfwWjGJsrwBjQJjTQLbnSTTmpTRQSTClHTbLmn
JqWWGvBJBwGJfJJGvwqZZddFtDFhgDqZhHNM
VwJcNgbfvfJbfcmGLZfPhZLfZGTDhP
CnnrlBlprsBnzQFntnZLqDhZZqThWGtWWSPL
FllFdCjzlsCzjJNJGGJwHHVg
fTbVBmNJCJRVbTmbfJFHsDjQHDHQjnQRsvDn
cLWcrGtttddMPhrPhPtPrtzsnSQQBvHjFpFSpDHsMjnvjD
PrPgPdhGWLrrqgdqcVCffbNblBwfVmwqwC
gmBfbmlbBDqrdfrDcJ
PwVWrQphQWWhQsJFcMPqzDdcJq
QWCSSHpSQWCttQpCRCHNSlZBtrmBZTjvGgZjmBZjBn
JrnhMPvtVtPVHJGrBrQwTmQmRGGB
pSSZCFClCbbSLbljZlSlFFszzBwcZNwTzQNDmBwGTBNTBz
ldsCCjpFjCqdLgsFjpsLFQgtnfqtJvnMtnvhWnHHMnnVWV
hzNHzHjWNzwHjjhprpGvGgvGvvpv
PLBVVRPDLdrgCdMrdrdC
FmBTqTmLPrsFqTBDcTTVtNNJztqWQNQtWWtJqNNz
fFffFvFBgHQWHdvfGglBWbqbPSSbSwVntPhZwwbS
jJCMzNMCjNCLNMjjphPSPqhbqnwPZLSqZh
rNpJJDzpcNMzzdBnGcQTccBvgv
FRFMwsrzVtwstgbCHHJJPgNb
hfZGhZDnnTTHTCCNzJjH
hppDvznmZphZQVFQwFVWlRqFls
jrjrgdHdFBZsBlcCGghWNgpgbCCp
QwJJqQQMLwPTwLMMwzvzwwzhCWbvcNcCChWpchWbNGfFff
qqFQJTmwJSPjZsrlnBjdHm
QfffRppWfHpQSrWVpSGmGMMccSjBjmmGmc
qdzLvbwzwdsWwnFdBBcBhMjMDvBBcBhc
bPdZPqddqzFsZVRptZZQHVWNpN
BzBQQHNjTSzzJDDFZFgJDJ
qLvCnLpfCpqCnLJhntRglFncDrGrllmZFZlDrc
JpqvfhRhLddfpbbtsdJWjHSwHHTNSQNPTVHQTb
qVQCCVlQZWgHZMqgqWlrtScFwrmtmcJqSSsSJS
MzdnddpNLzhRpzbzNPPBbPScjcnmrwSFjSjSJFtrwjcF
ddTzRMPLdLbvhBRdLWGQClVVCWQZQDTGGf
DHHTsldDNdPnVDCRDCNHllHwcMpprSMpRmphhRWhrhmzSS
qJLBqQLvJLQgftgPjJrhrMMWSmWMmMrrSqrc
FjfFftgLBjJPBLQZGvvZtNClnTTNGCdHTbCCNsnslH
jHHNsNqhjsShsshdRRCDMfMbCWHBrGGC
TJQFmnpgmTpBDCgCMCDZCGDC
FwpQzwQTmVvwTJmFJzTcQSdhBNztNPNjSlqLhBNhSh
vBCfSDcRMfRcRHSRRZZtPwrWWNtdSmrNVGSdwm
gbLnTzqTbjhGqFzgWrtttQtrPQTtNPmP
zbhjzglgzzlBGcsflsCl
jNHDNNHjVGVDNQFDTQSFZzDQTd
vvLwhbnpvPPgClwnfFTmTZQgffFFtTfc
LrhrLvwrnJvhCHVVRZNMjsRJVB
ShfcBWfvdhhJBBVwCJjHTRNwRVNC
qQzlDqMDDDslPqGVLTNZVpPwTRZZpV
bgbDbsqzsDTcfrgFFdgg
vlRHvvHwvMMMTTlvjmRtBjSJmSnDnpdrpSSrJJnDQrLp
cPfCgZZzNzzcGhNszcTPNZLrnVSJpJhDrrhSSQSpDDQn
FbTgbGcgNgcFbPFHMqvjjRtjRWvFvt
fZTnqfFFDNglcjdjZcfLGQJBwrGGQwbGQTBBJz
VhvfvsPpWRChmphvRGBbBLGhSbLrBQBSwb
pCsCsHvsstPsfRMMMtmDqFjngdFZqDHFnNFFjl
PVVwffMlfGWMDDSwfDwVpRpsZRjBHgpSsjJSpBSp
TdnFbqTFdmbjctcqcbRBZtJJZgsBzBBzvgHJ
bNmbcqnnbNFLChCVCfjDfWlMjVDPCr
JBLLjBQccLLJhcBDDlSrdFDsVhrVsR
HgNWCgqWGbvCRRZvGWvZmszsSlrWdSdFrWzSldDF
qvGCZGHggRNHvGTgvLnBjpjjPJwTPjLcJj
GCGwQrwBZMZdGVdLzbqbbp
TRfTTCtgcDmhtDmsTDVSbvpLdNpzNVRqVdVL
CfDJjscgTcsjfhtFZljPZWZMWPlZjQ
WsrjjfRfjjZjwjWjBpDpVpVhMBsMMSBT
JgmqHnCHHPCCtCJgSZMgZppDTgzvzZMz
HCGCGqqqCtmnHnqLFHWjlFrWRRbfjZccNWrR
BJBfSfPLPvdhvrbbvpDsHgDTzgpdzgZpgN
cVcmRnCWCqGngHpZsZsTsqNN
jmGCVwWjjnWFMjGwcwmrLJbBJPbLSrPPTbFBTr
SPZmmtlmqjZlZMwhlrtggqGGcCLCpfGLgqdCqF
FBBVDVTVDJfgcddLCDdp
zzVHvVNTbWJJTTRbVWBFJbWHmwmSPlMjPSShjlhMhhrrml
GJZJZTsnhsDJtVZdtsZJZrBCQpLjQgBnrQgjCjQQQj
RPSfqcRShHbFcPSfBqLLprBCwrQQQCqg
zPzPRHbFPcRRRHPclMhSfvfZsJZVTTZsJNVMGWGVdGTWWD
lCZrCLWCwVllGzWPPBMTFpsbGdsTpsbNMgFb
RDjtjHcHjcHctDRtjnhtnHTgMqTMqhTbdbdZbgFqZdMN
vfDmvfjtmvtcHmjZfSRZHQzBLrVLCJLJLfJBPzVJlwPw
JMTHVZMWNSCwCwMS
nsddQbDCnQQdDBPdCQCSvwpDvwffhfSvpmppvp
BssqBFtqRHgTqVRC
cWTTthtrgrzpCdCddtpz
SSSLNJLGLSLfCJfJFQCJzQ
swMPMZVMMSlMSZMqVSSHznzcqgzWTHgTnhbnrr
RJjjgMjWShPqchtbVBPV
DDddwCnZMHLLvDnfLrvvbVbbBtpwVBVPwtVpbcbb
zrvnvLrlZCHrfZZLffHZHHTsTmsQgFQSFTMjjQlFTRmR
zhTTMLRVTzLbVqwVRJgDQQsSCgCDNgsZCpqp
rrmrBmmWrWnHjWnGWrnGnhDHSQgNSpQsCgSNgtNtDDHZ
fBrGPGmGPBcTMfLhJVTc
TbTCjTBSbCncHsDZDZPhZbzv
rMwplFdlWWJMJzhhpGtHtvHSSP
fMMfwWdWrNfJNdlVgMcTLTmLffjTqnLScCjL
SwhTllwJDwqqBWLBbNtfhjBB
mvllZMmRMZGFZRfctLWtWttzfNLR
MGvHMCGpVnFGlgvVFFnpnGmmsHrDJJdSsqPqJSqDJJdTTDqD
QTTcqJZJhHSpShhFpFzjDDwwsFzpdg
NBMnBvmBPvwrqvgvvqgD
bNNGmWmbbClQTQRqchhQbf"#;
