#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the FPR
#
# URLS that need to be updated yearly:
# Main FPR
# RSNOPFKR
# COPF

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe
    import allpowerlifting_probe


MAIN_URLS = [
    'http://fpr-info.ru/protokol.htm',
    'http://fpr-info.ru/___protokoly/prot_2021/prot_2021.htm',
    'http://fpr-info.ru/___protokoly/prot_2019/prot_2019.htm',
    'http://fpr-info.ru/___protokoly/prot_2015/prot_2015.htm',
]

SPB_URL = 'http://www.powerliftingfed.spb.ru/protocols'

FPIO_URL = 'http://fpio.org.ru/index.php?menu=7'

FPKK_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPKK/'

FPK_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPK/'

PKFP_URL = 'http://ipf-perm.com/'

FPMO_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPMO/'

BPF_URL = 'http://powerbryansk.ru/protokols.shtml'

PFRR_URL = 'https://don-power.ru/%d0%bf%d1%80%d0%be%d1%82%d0%be%d0%ba%d0%be%d0%bb%d1%8b/'

MPF_URL = 'http://moscow-sila.ru/protokoly'

FPVO_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPVO/'

NSO_URL = 'http://fp-nso.ru/%d0%b4%d0%be%d0%ba%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d1%8b/'\
          '%d0%bf%d1%80%d0%be%d1%82%d0%be%d0%ba%d0%be%d0%bb%d1%8b/'

FPYNAO_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPYNAO/'

FPSK_URL = 'http://power.lekks.ru/?page_id=70'
FPSK_URL2 = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPSK/'

RSNOPFKR_URLS = ['http://xn----ptbjibfifei2b9dl.xn--p1ai/%D0%BF%D1%80%D0%BE%D1%82%D0%BE'
                 '%D0%BA%D0%BE%D0%BB%D1%8B/2020-%D0%B3%D0%BE%D0%B4-2/']

SFNRP_URLS = ['https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=0',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=40',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=80',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=120',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=160',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=200',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=240',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=280',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=320',
              'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai/index.php/protokoly'
              '?limitstart=360']

SBRPF_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/SBRPF/'

FPVRN_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPVrn/'

PFMR_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/PFMr/'

COPF_URLS = ['http://torpedo74.ru/palace/514/#content',
             'http://torpedo74.ru/palace/522/#content']

KPF_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/KPF/'

TPF_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/TPF/'

FPRSYA_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPRSYA/'

FPRME_URL = 'https://en.allpowerlifting.com/federations/IPF/RPF/FPRME/'

PFT_URL = 'http://fpr71.ru/%d0%bf%d1%80%d0%be%d1%82%d0%be%d0%ba%d0%be%d0%bb%d1%8b/'


FEDDIR = os.path.dirname(os.path.realpath(__file__))


def colour(s):
    return "\033[1;33m" + s + "\033[0;m"


# FPR has malformed html on their results page, remove the extra tags
def fix_html(html):
    tag_dict = {}
    remove_tags = []

    ii = 0
    split_html = html.split('<')

    # Find unopened tags
    for tag_start in split_html:
        if tag_start != '':
            if tag_start[0] == '/':  # Close tag
                close_idx = tag_start.find(">")
                tag_type = tag_start[1:tag_start.find(">")]
                if tag_type not in tag_dict:
                    remove_tags.append(ii)
                elif tag_dict[tag_type] == 0:
                    remove_tags.append(ii)
                else:
                    tag_dict[tag_type] -= 1

            elif '/>' not in tag_start and tag_start[0] != 'p':  # Open tag
                close_idx = tag_start.find(">")
                tag_type = tag_start[0:tag_start.find(">")].split(" ")[0]
                if tag_type not in tag_dict:
                    tag_dict[tag_type] = 1
                else:
                    tag_dict[tag_type] += 1
        ii = ii + 1

    # Remove the unopened tags
    for idx in remove_tags:
        close_idx = split_html[idx].find(">")
        if len(split_html[idx]) > close_idx + 1:
            split_html[idx] = split_html[idx][close_idx + 1:]
        else:
            split_html[idx] = ''

    split_html = ["<" + line for line in split_html if line != '']
    return ''.join(split_html)


def getmeetlist(html, main_url):
    base_url = main_url.rsplit('/', 1)[0]

    html = fix_html(str(html, 'windows-1251'))

    soup = BeautifulSoup(html, 'html.parser')
    divs = soup.find_all("table", {"width": "101%"})

    # Pre 2010 the tables are slightly different
    if divs == []:
        divs = soup.find_all("table", {"id": "table2"})  # 07
    if divs == []:
        divs = soup.find_all("table", {"id": ["table7", "table4"]})  # 08
    if divs == []:
        divs = soup.find_all("table", {"id": ["table11", "table12"]})  # 09
    if divs == []:
        divs = soup.find_all("table", {"width": "102%"})

    urls = []
    for div in divs:
        for a in div.find_all('a'):
            url = a['href']

        # Non relative links are meet photos and international meets, mir
        # =Worlds,evr =Euros
            if not any(test_str in url for test_str in ['http', 'mir', 'arnold', 'evr']):
                url = base_url + "/" + url
                url = url.replace('.com//', '/')
                if url not in urls:
                    urls.append(url)
    return urls


def getmeetlist_SPB(html, main_url):
    base_url = main_url.rsplit('/', 1)[0]
    soup = BeautifulSoup(html, 'html.parser')
    divs = soup.find_all('div', {'class', 'list-group'})
    urls = []
    for div in divs:
        for a in div.find_all('a'):
            url = a['href']
            url = base_url + "/" + url
            url = url.replace('.ru//', '.ru/')
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_FPIO(html, main_url):
    base_url = main_url.rsplit('/', 1)[0]
    soup = BeautifulSoup(html, 'html.parser')
    divs = soup.find_all('div', {'class', 'doc_block_max'})
    urls = []
    for div in divs:
        for a in div.find_all('a'):
            url = a['href']
            url = base_url + "/" + url
            url = url.replace('.ru//', '.ru/')
            if 'allpowerlifting' in url:
                continue
            elif '.rus.' in url or '-rus-' in url:
                continue
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_NSO(html, main_url):
    soup = BeautifulSoup(html, 'html.parser')
    divs = soup.find_all('div', {'class', 'entry-content'})
    urls = []
    for div in divs:
        for a in div.find_all('a'):
            url = a['href']
            if 'addtoany' in url:
                continue
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_PKFP(html, main_url):
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 't585'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']
            if 'yadi.sk' not in url:
                continue
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_MPF(html, main_url):
    soup = BeautifulSoup(html, 'html.parser')
    article = soup.find('article', {'id', 'post-7'})
    urls = []
    for a in article.find_all('a'):
        if a.has_attr('href'):
            url = a['href']
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_BPF(html, main_url):
    base_url = main_url.rsplit('/', 1)[0]
    soup = BeautifulSoup(html, 'html.parser')
    td = soup.find('td', {'class', 'ver8nn_12'})
    urls = []
    for a in td.find_all('a'):

        if a.has_attr('href'):
            url = a['href']
            if 'http' in url:
                continue
            elif 'euro' in url.lower():
                continue
            elif 'world' in url.lower():
                continue
            elif 'evro' in url.lower():
                continue
            elif 'evr_' in url.lower():
                continue
            elif 'ChE' in url:
                continue
            elif 'mir_' in url.lower():
                continue
            url = base_url + "/" + url

            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_PFRR(html, main_url):
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 'entry-container'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_FPSK(html, main_url):
    base_url = main_url.rsplit('/', 1)[0]
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 'entry-content'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']

            url = base_url + url
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_RSNOPFKR(html, main_url):
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 'site-content'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']

            if 'wp-login.php?redirect_to=' in url:
                continue
            elif 'respond' in url:
                continue
            elif 'ipf' in url.lower():
                continue
            elif 'epf' in url.lower():
                continue

            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_SFNRP(html, main_url):
    base_url = 'https://xn----8sbhccegpecailstnhaynip6au2a4u.xn--p1ai'
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 'grid_10'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']

            url = base_url + url
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_COPF(html, main_url):
    base_url = 'http://torpedo74.ru'
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 'text-content'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']
            if 'data' not in url:
                continue

            url = base_url + url
            if url not in urls:
                urls.append(url)
    return urls


def getmeetlist_PFT(html, main_url):
    soup = BeautifulSoup(html, 'html.parser')
    div = soup.find('div', {'class', 'smc-postcontent'})
    urls = []
    for a in div.find_all('a'):
        if a.has_attr('href'):
            url = a['href']

            if 'fpr71' not in url:
                continue

            if url not in urls:
                urls.append(url)
    return urls


def main():

    meetlist = []
    for url in MAIN_URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html, url)

    html_SPB = oplprobe.gethtml(SPB_URL)
    meetlist = meetlist + getmeetlist_SPB(html_SPB, SPB_URL)

    html_FPIO = oplprobe.gethtml(FPIO_URL)
    meetlist = meetlist + getmeetlist_FPIO(html_FPIO, FPIO_URL)

    html_NSO = oplprobe.gethtml(NSO_URL)
    meetlist = meetlist + getmeetlist_NSO(html_NSO, NSO_URL)

#    html_PKFP = oplprobe.gethtml(PKFP_URL)
#    meetlist = meetlist + getmeetlist_PKFP(html_PKFP, PKFP_URL)

    html_MPF = oplprobe.gethtml(MPF_URL)
    meetlist = meetlist + getmeetlist_MPF(html_MPF, MPF_URL)

    html_BPF = oplprobe.gethtml(BPF_URL)
    meetlist = meetlist + getmeetlist_BPF(html_BPF, BPF_URL)

    html_PFRR = oplprobe.gethtml(PFRR_URL)
    meetlist = meetlist + getmeetlist_PFRR(html_PFRR, PFRR_URL)

    meetlist = meetlist + allpowerlifting_probe.probefederation(FPKK_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPK_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPMO_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPVO_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPYNAO_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPSK_URL2)
    meetlist = meetlist + allpowerlifting_probe.probefederation(SBRPF_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPVRN_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(PFMR_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(KPF_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(TPF_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPRSYA_URL)
    meetlist = meetlist + allpowerlifting_probe.probefederation(FPRME_URL)

    html_FPSK = oplprobe.gethtml(FPSK_URL)
    meetlist = meetlist + getmeetlist_FPSK(html_FPSK, FPSK_URL)

    for url in RSNOPFKR_URLS:
        html_RSNOPFKR = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist_RSNOPFKR(html_RSNOPFKR, url)

    for url in SFNRP_URLS:
        html_SFNRP = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist_SFNRP(html_SFNRP, url)
    for url in COPF_URLS:
        html_COPF = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist_COPF(html_COPF, url)

    html_PFT = oplprobe.gethtml(PFT_URL)
    meetlist = meetlist + getmeetlist_PFT(html_PFT, PFT_URL)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(colour('[FPR]'), unentered)


if __name__ == '__main__':
    main()
