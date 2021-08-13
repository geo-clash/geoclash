def format_amount(a):
	return a.replace(",","").strip().replace("%","").replace("$","")

def create_json(gdp, coords):

	# ------------ Split gdp data ------------ #
	line_list=gdp.split('\n')
	column_list = [x.split('\t') for x in line_list if x!=""]

	# ------------ Split coord data ------------ #
	line_list=coords.split('\n')
	coord_list = [x.split(',') for x in line_list if x!=""]
	coord_dict = {}
	for i in coord_list:
		coord_dict[format_amount(i[0])] = i[1:]


	# ------------ Begin File ------------ #
	out = "// This file is automatically generated by game-statics/utils/countryRON.py.\n// Please do not edit."
	out += "\n["

	# -------- Add country list -------- #
	for index in range(len(column_list)):
		coords = coord_dict[format_amount(column_list[index][1]) ]
		print(coords)

		out += "("
		out+='name:"'        + format_amount(column_list[index][1]) + '",'
		out+='gdp:'          + format_amount(column_list[index][2]) + ','
		out+='population:'   + format_amount(column_list[index][5]) + ','
		out+='lat:'          + format_amount(coords            [1]) + ','
		out+='long:'         + format_amount(coords            [2]) + ''
		out+=")"
		if index!=len(column_list)-1:
			out+=','

	# ----------- End File ----------- #
	out+="]"


	

	return out

def create_file():
	data = create_json(d, coords)
	file = open("../assets/Countries.ron","w",encoding='utf8') 
 
	file.write(data) 
 
	file.close() 

# Copied from https://www.worldometers.info/gdp/gdp-by-country/
#	Country			GDP					GDP formated		GDP change	Population	GDP per capita share of word GDP
d='''
1	 United States	 $19,485,394,000,000	 $19.485 trillion	 2.27%	 325,084,756	 $59,939	 24.08%
2	 China	 $12,237,700,479,375	 $12.238 trillion	 6.90%	 1,421,021,791	 $8,612	 15.12%
3	 Japan	 $4,872,415,104,315	 $4.872 trillion	 1.71%	 127,502,725	 $38,214	 6.02%
4	 Germany	 $3,693,204,332,230	 $3.693 trillion	 2.22%	 82,658,409	 $44,680	 4.56%
5	 India	 $2,650,725,335,364	 $2.651 trillion	 6.68%	 1,338,676,785	 $1,980	 3.28%
6	 United Kingdom	 $2,637,866,340,434	 $2.638 trillion	 1.79%	 66,727,461	 $39,532	 3.26%
7	 France	 $2,582,501,307,216	 $2.583 trillion	 1.82%	 64,842,509	 $39,827	 3.19%
8	 Brazil	 $2,053,594,877,013	 $2.054 trillion	 0.98%	 207,833,823	 $9,881	 2.54%
9	 Italy	 $1,943,835,376,342	 $1.944 trillion	 1.50%	 60,673,701	 $32,038	 2.40%
10	 Canada	 $1,647,120,175,449	 $1.647 trillion	 3.05%	 36,732,095	 $44,841	 2.04%
11	 Russia	 $1,578,417,211,937	 $1.578 trillion	 1.55%	 145,530,082	 $10,846	 1.95%
12	 South Korea	 $1,530,750,923,149	 $1.531 trillion	 3.06%	 51,096,415	 $29,958	 1.89%
13	 Australia	 $1,323,421,072,479	 $1.323 trillion	 1.96%	 24,584,620	 $53,831	 1.64%
14	 Spain	 $1,314,314,164,402	 $1.314 trillion	 3.05%	 46,647,428	 $28,175	 1.62%
15	 Mexico	 $1,150,887,823,404	 $1.151 trillion	 2.04%	 124,777,324	 $9,224	 1.42%
16	 Indonesia	 $1,015,420,587,285	 $1.015 trillion	 5.07%	 264,650,963	 $3,837	 1.25%
17	 Turkey	 $851,549,299,635	 $852 billion	 7.44%	 81,116,450	 $10,498	 1.05%
18	 Netherlands	 $830,572,618,850	 $831 billion	 3.16%	 17,021,347	 $48,796	 1.03%
19	 Saudi Arabia	 $686,738,400,000	 $687 billion	 -0.86%	 33,101,179	 $20,747	 0.85%
20	 Switzerland	 $678,965,423,322	 $679 billion	 1.09%	 8,455,804	 $80,296	 0.84%
21	 Argentina	 $637,430,331,479	 $637 billion	 2.85%	 43,937,140	 $14,508	 0.79%
22	 Sweden	 $535,607,385,506	 $536 billion	 2.29%	 9,904,896	 $54,075	 0.66%
23	 Poland	 $526,465,839,003	 $526 billion	 4.81%	 37,953,180	 $13,871	 0.65%
24	 Belgium	 $494,763,551,891	 $495 billion	 1.73%	 11,419,748	 $43,325	 0.61%
25	 Thailand	 $455,302,682,986	 $455 billion	 3.91%	 69,209,810	 $6,579	 0.56%
26	 Iran	 $454,012,768,724	 $454 billion	 3.76%	 80,673,883	 $5,628	 0.56%
27	 Austria	 $416,835,975,862	 $417 billion	 3.04%	 8,819,901	 $47,261	 0.52%
28	 Norway	 $399,488,897,844	 $399 billion	 1.92%	 5,296,326	 $75,428	 0.49%
29	 United Arab Emirates	 $382,575,085,092	 $383 billion	 0.79%	 9,487,203	 $40,325	 0.47%
30	 Nigeria	 $375,745,486,521	 $376 billion	 0.81%	 190,873,244	 $1,969	 0.46%
31	 Israel	 $353,268,411,919	 $353 billion	 3.33%	 8,243,848	 $42,852	 0.44%
32	 South Africa	 $348,871,647,960	 $349 billion	 1.32%	 57,009,756	 $6,120	 0.43%
33	 Hong Kong	 $341,449,340,451	 $341 billion	 3.79%	 7,306,322	 $46,733	 0.42%
34	 Ireland	 $331,430,014,003	 $331 billion	 7.80%	 4,753,279	 $69,727	 0.41%
35	 Denmark	 $329,865,537,183	 $330 billion	 2.24%	 5,732,274	 $57,545	 0.41%
36	 Singapore	 $323,907,234,412	 $324 billion	 3.62%	 5,708,041	 $56,746	 0.40%
37	 Malaysia	 $314,710,259,511	 $315 billion	 5.90%	 31,104,646	 $10,118	 0.39%
38	 Colombia	 $314,457,601,860	 $314 billion	 1.79%	 48,909,839	 $6,429	 0.39%
39	 Philippines	 $313,595,208,737	 $314 billion	 6.68%	 105,172,925	 $2,982	 0.39%
40	 Pakistan	 $304,951,818,494	 $305 billion	 5.70%	 207,906,209	 $1,467	 0.38%
41	 Chile	 $277,075,944,402	 $277 billion	 1.49%	 18,470,439	 $15,001	 0.34%
42	 Finland	 $252,301,837,573	 $252 billion	 2.63%	 5,511,371	 $45,778	 0.31%
43	 Bangladesh	 $249,723,862,487	 $250 billion	 7.28%	 159,685,424	 $1,564	 0.31%
44	 Egypt	 $235,369,129,338	 $235 billion	 4.18%	 96,442,591	 $2,441	 0.29%
45	 Vietnam	 $223,779,865,815	 $224 billion	 6.81%	 94,600,648	 $2,366	 0.28%
46	 Portugal	 $219,308,128,887	 $219 billion	 2.68%	 10,288,527	 $21,316	 0.27%
47	 Czech Republic	 $215,913,545,038	 $216 billion	 4.29%	 10,641,034	 $20,291	 0.27%
48	 Romania	 $211,883,923,504	 $212 billion	 7.26%	 19,653,969	 $10,781	 0.26%
49	 Peru	 $211,389,272,242	 $211 billion	 2.53%	 31,444,298	 $6,723	 0.26%
50	 New Zealand	 $204,139,049,909	 $204 billion	 3.03%	 4,702,034	 $43,415	 0.25%
51	 Greece	 $203,085,551,429	 $203 billion	 1.35%	 10,569,450	 $19,214	 0.25%
52	 Iraq	 $192,060,810,811	 $192 billion	 -2.07%	 37,552,781	 $5,114	 0.24%
53	 Algeria	 $167,555,280,113	 $168 billion	 1.60%	 41,389,189	 $4,048	 0.21%
54	 Qatar	 $166,928,571,429	 $167 billion	 1.58%	 2,724,728	 $61,264	 0.21%
55	 Kazakhstan	 $162,886,867,832	 $163 billion	 4.10%	 18,080,019	 $9,009	 0.20%
56	 Hungary	 $139,761,138,103	 $140 billion	 3.99%	 9,729,823	 $14,364	 0.17%
57	 Angola	 $122,123,822,334	 $122 billion	 -0.15%	 29,816,766	 $4,096	 0.15%
58	 Kuwait	 $120,126,277,613	 $120 billion	 -2.87%	 4,056,099	 $29,616	 0.15%
59	 Sudan	 $117,487,857,143	 $117 billion	 4.28%	 40,813,397	 $2,879	 0.15%
60	 Ukraine	 $112,154,185,121	 $112 billion	 2.52%	 44,487,709	 $2,521	 0.14%
61	 Morocco	 $109,708,728,849	 $110 billion	 4.09%	 35,581,255	 $3,083	 0.14%
62	 Ecuador	 $104,295,862,000	 $104 billion	 2.37%	 16,785,361	 $6,214	 0.13%
63	 Cuba	 $96,851,000,000	 $96.85 billion	 1.78%	 11,339,254	 $8,541	 0.12%
64	 Slovakia	 $95,617,670,260	 $95.62 billion	 3.40%	 5,447,900	 $17,551	 0.12%
65	 Sri Lanka	 $87,357,205,923	 $87.36 billion	 3.31%	 21,128,032	 $4,135	 0.11%
66	 Ethiopia	 $80,561,496,134	 $80.56 billion	 10.25%	 106,399,924	 $757	 0.10%
67	 Kenya	 $79,263,075,749	 $79.26 billion	 4.87%	 50,221,142	 $1,578	 0.10%
68	 Dominican Republic	 $75,931,656,815	 $75.93 billion	 4.55%	 10,513,104	 $7,223	 0.09%
69	 Guatemala	 $75,620,095,538	 $75.62 billion	 2.76%	 16,914,970	 $4,471	 0.09%
70	 Oman	 $70,783,875,163	 $70.78 billion	 -0.27%	 4,665,928	 $15,170	 0.09%
71	 Myanmar	 $67,068,745,521	 $67.07 billion	 6.76%	 53,382,523	 $1,256	 0.08%
72	 Luxembourg	 $62,316,359,824	 $62.32 billion	 2.30%	 591,910	 $105,280	 0.08%
73	 Panama	 $62,283,756,584	 $62.28 billion	 5.32%	 4,106,769	 $15,166	 0.08%
74	 Ghana	 $58,996,776,238	 $59.00 billion	 8.14%	 29,121,465	 $2,026	 0.07%
75	 Bulgaria	 $58,220,973,783	 $58.22 billion	 3.81%	 7,102,444	 $8,197	 0.07%
76	 Costa Rica	 $57,285,984,448	 $57.29 billion	 3.28%	 4,949,954	 $11,573	 0.07%
77	 Uruguay	 $56,156,972,158	 $56.16 billion	 2.66%	 3,436,641	 $16,341	 0.07%
78	 Croatia	 $55,213,087,271	 $55.21 billion	 2.92%	 4,182,857	 $13,200	 0.07%
79	 Belarus	 $54,456,465,473	 $54.46 billion	 2.42%	 9,450,231	 $5,762	 0.07%
80	 Lebanon	 $53,576,985,687	 $53.58 billion	 1.53%	 6,819,373	 $7,857	 0.07%
81	 Tanzania	 $53,320,625,959	 $53.32 billion	 7.10%	 54,660,339	 $975	 0.07%
82	 Macau	 $50,361,201,096	 $50.36 billion	 9.10%	 622,585	 $80,890	 0.06%
83	 Uzbekistan	 $49,677,172,714	 $49.68 billion	 5.30%	 31,959,785	 $1,554	 0.06%
84	 Slovenia	 $48,769,655,479	 $48.77 billion	 5.00%	 2,076,394	 $23,488	 0.06%
85	 Lithuania	 $47,544,459,559	 $47.54 billion	 3.83%	 2,845,414	 $16,709	 0.06%
86	 Serbia	 $41,431,648,801	 $41.43 billion	 1.87%	 8,829,628	 $4,692	 0.05%
87	 Azerbaijan	 $40,747,792,238	 $40.75 billion	 0.10%	 9,845,320	 $4,139	 0.05%
88	 Jordan	 $40,068,308,451	 $40.07 billion	 1.97%	 9,785,843	 $4,095	 0.05%
89	 Tunisia	 $39,952,095,561	 $39.95 billion	 1.96%	 11,433,443	 $3,494	 0.05%
90	 Paraguay	 $39,667,400,816	 $39.67 billion	 5.21%	 6,867,061	 $5,776	 0.05%
91	 Libya	 $38,107,728,083	 $38.11 billion	 26.68%	 6,580,724	 $5,791	 0.05%
92	 Turkmenistan	 $37,926,285,714	 $37.93 billion	 6.50%	 5,757,667	 $6,587	 0.05%
93	 DR Congo	 $37,642,482,562	 $37.64 billion	 3.70%	 81,398,764	 $462	 0.05%
94	 Bolivia	 $37,508,642,113	 $37.51 billion	 4.20%	 11,192,855	 $3,351	 0.05%
95	 Côte d'Ivoire	 $37,353,276,059	 $37.35 billion	 7.70%	 24,437,470	 $1,529	 0.05%
96	 Bahrain	 $35,432,686,170	 $35.43 billion	 3.88%	 1,494,076	 $23,715	 0.04%
97	 Cameroon	 $34,922,782,311	 $34.92 billion	 3.55%	 24,566,073	 $1,422	 0.04%
98	 Yemen	 $31,267,675,216	 $31.27 billion	 -5.94%	 27,834,819	 $1,123	 0.04%
99	 Latvia	 $30,463,302,414	 $30.46 billion	 4.55%	 1,951,097	 $15,613	 0.04%
100	 Estonia	 $26,611,651,599	 $26.61 billion	 4.85%	 1,319,390	 $20,170	 0.03%
101	 Uganda	 $25,995,031,850	 $26.00 billion	 3.86%	 41,166,588	 $631	 0.03%
102	 Zambia	 $25,868,142,073	 $25.87 billion	 3.40%	 16,853,599	 $1,535	 0.03%
103	 Nepal	 $24,880,266,905	 $24.88 billion	 7.91%	 27,632,681	 $900	 0.03%
104	 El Salvador	 $24,805,439,600	 $24.81 billion	 2.32%	 6,388,126	 $3,883	 0.03%
105	 Iceland	 $24,488,467,010	 $24.49 billion	 3.64%	 334,393	 $73,233	 0.03%
106	 Honduras	 $22,978,532,897	 $22.98 billion	 4.79%	 9,429,013	 $2,437	 0.03%
107	 Cambodia	 $22,158,209,503	 $22.16 billion	 7.10%	 16,009,409	 $1,384	 0.03%
108	 Trinidad and Tobago	 $22,079,017,627	 $22.08 billion	 -2.34%	 1,384,059	 $15,952	 0.03%
109	 Cyprus	 $22,054,225,828	 $22.05 billion	 4.23%	 1,179,678	 $18,695	 0.03%
110	 Zimbabwe	 $22,040,902,300	 $22.04 billion	 4.70%	 14,236,595	 $1,548	 0.03%
111	 Senegal	 $21,070,225,735	 $21.07 billion	 7.15%	 15,419,355	 $1,366	 0.03%
112	 Papua New Guinea	 $20,536,314,601	 $20.54 billion	 2.55%	 8,438,036	 $2,434	 0.03%
113	 Afghanistan	 $19,543,976,895	 $19.54 billion	 2.67%	 36,296,113	 $538	 0.02%
114	 Bosnia and Herzegovina	 $18,054,854,789	 $18.05 billion	 3.19%	 3,351,525	 $5,387	 0.02%
115	 Botswana	 $17,406,565,823	 $17.41 billion	 2.36%	 2,205,080	 $7,894	 0.02%
116	 Laos	 $16,853,087,485	 $16.85 billion	 6.89%	 6,953,035	 $2,424	 0.02%
117	 Mali	 $15,334,336,144	 $15.33 billion	 5.40%	 18,512,430	 $828	 0.02%
118	 Georgia	 $15,081,338,092	 $15.08 billion	 4.83%	 4,008,716	 $3,762	 0.02%
119	 Gabon	 $15,013,950,984	 $15.01 billion	 0.50%	 2,064,823	 $7,271	 0.02%
120	 Jamaica	 $14,781,107,822	 $14.78 billion	 0.98%	 2,920,848	 $5,061	 0.02%
121	 Palestine	 $14,498,100,000	 $14.50 billion	 3.14%	 4,747,227	 $3,054	 0.02%
122	 Nicaragua	 $13,814,261,536	 $13.81 billion	 4.86%	 6,384,846	 $2,164	 0.02%
123	 Mauritius	 $13,266,427,697	 $13.27 billion	 3.82%	 1,264,499	 $10,491	 0.02%
124	 Namibia	 $13,253,698,015	 $13.25 billion	 -0.95%	 2,402,633	 $5,516	 0.02%
125	 Albania	 $13,038,538,300	 $13.04 billion	 3.84%	 2,884,169	 $4,521	 0.02%
126	 Mozambique	 $12,645,508,634	 $12.65 billion	 3.74%	 28,649,018	 $441	 0.02%
127	 Malta	 $12,518,134,319	 $12.52 billion	 6.42%	 437,933	 $28,585	 0.02%
128	 Burkina Faso	 $12,322,864,245	 $12.32 billion	 6.30%	 19,193,234	 $642	 0.02%
129	 Equatorial Guinea	 $12,293,579,173	 $12.29 billion	 -4.92%	 1,262,002	 $9,741	 0.02%
130	 Bahamas	 $12,162,100,000	 $12.16 billion	 1.44%	 381,755	 $31,858	 0.02%
131	 Brunei	 $12,128,089,002	 $12.13 billion	 1.33%	 424,473	 $28,572	 0.01%
132	 Armenia	 $11,536,590,636	 $11.54 billion	 7.50%	 2,944,791	 $3,918	 0.01%
133	 Madagascar	 $11,499,803,807	 $11.50 billion	 4.17%	 25,570,512	 $450	 0.01%
134	 Mongolia	 $11,433,635,876	 $11.43 billion	 5.30%	 3,113,786	 $3,672	 0.01%
135	 North Macedonia	 $11,279,509,014	 $11.28 billion	 0.24%	 2,081,996	 $5,418	 0.01%
136	 Guinea	 $10,472,514,515	 $10.47 billion	 10.60%	 12,067,519	 $868	 0.01%
137	 Chad	 $9,871,247,732	 $9.87 billion	 -2.95%	 15,016,753	 $657	 0.01%
138	 Benin	 $9,246,696,924	 $9.25 billion	 5.84%	 11,175,198	 $827	 0.01%
139	 Rwanda	 $9,135,454,442	 $9.14 billion	 6.06%	 11,980,961	 $762	 0.01%
140	 Congo	 $8,701,334,800	 $8.70 billion	 -3.10%	 5,110,695	 $1,703	 0.01%
141	 Haiti	 $8,408,150,518	 $8.41 billion	 1.17%	 10,982,366	 $766	 0.01%
142	 Moldova	 $8,128,493,432	 $8.13 billion	 4.50%	 4,059,684	 $2,002	 0.01%
143	 Niger	 $8,119,710,126	 $8.12 billion	 4.89%	 21,602,382	 $376	 0.01%
144	 Kyrgyzstan	 $7,564,738,836	 $7.56 billion	 4.58%	 6,189,733	 $1,222	 0.01%
145	 Tajikistan	 $7,146,449,583	 $7.15 billion	 7.62%	 8,880,268	 $805	 0.01%
146	 Malawi	 $6,303,292,264	 $6.30 billion	 4.00%	 17,670,196	 $357	 0.01%
147	 Guam	 $5,859,000,000	 $5.86 billion	 0.19%	 164,281	 $35,665	 0.01%
148	 Fiji	 $5,061,202,767	 $5.06 billion	 3.80%	 877,459	 $5,768	 0.01%
149	 Mauritania	 $5,024,708,656	 $5.02 billion	 3.50%	 4,282,570	 $1,173	 0.01%
150	 Maldives	 $4,865,546,027	 $4.87 billion	 6.91%	 496,402	 $9,802	 0.01%
151	 Montenegro	 $4,844,592,067	 $4.84 billion	 4.70%	 627,563	 $7,720	 0.01%
152	 Togo	 $4,757,776,485	 $4.76 billion	 4.40%	 7,698,474	 $618	 0.01%
153	 Barbados	 $4,673,500,000	 $4.67 billion	 1.00%	 286,232	 $16,328	 0.01%
154	 Eswatini	 $4,433,664,364	 $4.43 billion	 1.87%	 1,124,805	 $3,942	 0.01%
155	 Sierra Leone	 $3,775,047,334	 $3.78 billion	 4.21%	 7,488,423	 $504	 0.00%
156	 Guyana	 $3,621,046,005	 $3.62 billion	 2.92%	 775,222	 $4,671	 0.00%
157	 Liberia	 $3,285,455,000	 $3.29 billion	 2.47%	 4,702,226	 $699	 0.00%
158	 Burundi	 $3,172,416,146	 $3.17 billion	 0.50%	 10,827,019	 $293	 0.00%
159	 Andorra	 $3,012,914,131	 $3.01 billion	 1.87%	 77,001	 $39,128	 0.00%
160	 Suriname	 $2,995,827,901	 $3.00 billion	 1.69%	 570,496	 $5,251	 0.00%
161	 Timor-Leste	 $2,954,621,000	 $2.95 billion	 -8.00%	 1,243,258	 $2,377	 0.00%
162	 Aruba	 $2,700,558,659	 $2.70 billion	 1.33%	 105,366	 $25,630	 0.00%
163	 Lesotho	 $2,578,265,358	 $2.58 billion	 -2.29%	 2,091,534	 $1,233	 0.00%
164	 Bhutan	 $2,528,007,911	 $2.53 billion	 4.63%	 745,563	 $3,391	 0.00%
165	 Central African Republic	 $1,949,411,659	 $1.95 billion	 4.30%	 4,596,023	 $424	 0.00%
166	 Belize	 $1,862,614,800	 $1.86 billion	 1.44%	 375,769	 $4,957	 0.00%
167	 Cape Verde	 $1,772,706,451	 $1.77 billion	 4.01%	 537,498	 $3,298	 0.00%
168	 Saint Lucia	 $1,737,504,296	 $1.74 billion	 3.82%	 180,954	 $9,602	 0.00%
169	 San Marino	 $1,632,860,041	 $1.63 billion	 1.50%	 33,671	 $48,495	 0.00%
170	 Northern Mariana Islands	 $1,593,000,000	 $1.59 billion	 25.14%	 56,562	 $28,164	 0.00%
171	 Antigua and Barbuda	 $1,510,084,751	 $1.51 billion	 3.03%	 95,426	 $15,825	 0.00%
172	 Seychelles	 $1,497,959,569	 $1.50 billion	 5.28%	 96,418	 $15,536	 0.00%
173	 Gambia	 $1,489,464,788	 $1.49 billion	 4.56%	 2,213,889	 $673	 0.00%
174	 Guinea-Bissau	 $1,346,841,897	 $1.35 billion	 5.92%	 1,828,145	 $737	 0.00%
175	 Solomon Islands	 $1,303,453,622	 $1.30 billion	 3.24%	 636,039	 $2,049	 0.00%
176	 Grenada	 $1,126,882,296	 $1.13 billion	 5.06%	 110,874	 $10,164	 0.00%
177	 Comoros	 $1,068,124,330	 $1.07 billion	 2.71%	 813,892	 $1,312	 0.00%
178	 Saint Kitts and Nevis	 $992,007,403	 $992 million	 1.17%	 52,045	 $19,061	 0.00%
179	 Vanuatu	 $862,879,789	 $863 million	 4.50%	 285,510	 $3,022	 0.00%
180	 Samoa	 $840,927,997	 $841 million	 2.70%	 195,352	 $4,305	 0.00%
181	 Saint Vincent and the Grenadines	 $785,222,509	 $785 million	 0.86%	 109,827	 $7,150	 0.00%
182	 American Samoa	 $634,000,000	 $634 million	 -5.38%	 55,620	 $11,399	 0.00%
183	 Dominica	 $496,727,000	 $497 million	 -9.53%	 71,458	 $6,951	 0.00%
184	 Tonga	 $427,659,795	 $428 million	 2.70%	 101,998	 $4,193	 0.00%
185	 São Tomé and Príncipe	 $392,570,293	 $393 million	 3.87%	 207,089	 $1,896	 0.00%
186	 Micronesia	 $336,427,500	 $336 million	 3.20%	 532,899	 $631	 0.00%
187	 Palau	 $289,823,500	 $290 million	 -3.57%	 17,808	 $16,275	 0.00%
188	 Marshall Islands	 $204,173,430	 $204 million	 3.60%	 58,058	 $3,517	 0.00%
189	 Kiribati	 $185,572,502	 $186 million	 0.33%	 114,158	 $1,626	 0.00%
190	 Tuvalu	 $39,731,317	 $40 million	 3.24%	 11,370	 $3,494	 0.00%'''

coords = '''Abkhazia,Sukhumi,43.001525,41.023415
Afghanistan,Kabul,34.575503,69.240073
Aland Islands,Mariehamn,60.1,19.933333
Albania,Tirana,41.327546,19.818698
Algeria,Algiers,36.752887,3.042048
American Samoa,Pago Pago,-14.275632,-170.702036
Andorra,Andorra la Vella,42.506317,1.521835
Angola,Luanda,-8.839988,13.289437
Anguilla,The Valley,18.214813,-63.057441
Antarctica,South Pole,-90,0
Antigua and Barbuda,Saint John's,17.12741,-61.846772
Argentina,Buenos Aires,-34.603684,-58.381559
Armenia,Yerevan,40.179186,44.499103
Aruba,Oranjestad,12.509204,-70.008631
Australia,Canberra,-35.282,149.128684
Austria,Vienna,48.208174,16.373819
Azerbaijan,Baku,40.409262,49.867092
Bahamas,Nassau,25.047984,-77.355413
Bahrain,Manama,26.228516,50.58605
Bangladesh,Dhaka,23.810332,90.412518
Barbados,Bridgetown,13.113222,-59.598809
Belarus,Minsk,53.90454,27.561524
Belgium,Brussels,50.85034,4.35171
Belize,Belmopan,17.251011,-88.75902
Benin,Porto-Novo,6.496857,2.628852
Bermuda,Hamilton,32.294816,-64.781375
Bhutan,Thimphu,27.472792,89.639286
Bolivia,La Paz,-16.489689,-68.119294
Bosnia and Herzegovina,Sarajevo,43.856259,18.413076
Botswana,Gaborone,-24.628208,25.923147
Bouvet Island,Bouvet Island,-54.43,3.38
Brazil,Brasília,-15.794229,-47.882166
British Indian Ocean Territory,Camp Justice,21.3419,55.4778
British Virgin Islands,Road Town,18.428612,-64.618466
Brunei,Bandar Seri Begawan,4.903052,114.939821
Bulgaria,Sofia,42.697708,23.321868
Burkina Faso,Ouagadougou,12.371428,-1.51966
Burundi,Bujumbura,-3.361378,29.359878
Cambodia,Phnom Penh,11.544873,104.892167
Cameroon,Yaoundé,3.848033,11.502075
Canada,Ottawa,45.42153,-75.697193
Cape Verde,Praia,14.93305,-23.513327
Cayman Islands,George Town,19.286932,-81.367439
Central African Republic,Bangui,4.394674,18.55819
Chad,N'Djamena,12.134846,15.055742
Chile,Santiago,-33.44889,-70.669265
China,Beijing,39.904211,116.407395
Christmas Island,Flying Fish Cove,-10.420686,105.679379
Cocos (Keeling) Islands,West Island,-12.188834,96.829316
Colombia,Bogotá,4.710989,-74.072092
Comoros,Moroni,-11.717216,43.247315
DR Congo,Kinshasa,-4.441931,15.266293
Congo,Brazzaville,-4.26336,15.242885
Cook Islands,Avarua,-21.212901,-159.782306
Costa Rica,San José,9.928069,-84.090725
Côte d'Ivoire,Yamoussoukro,6.827623,-5.289343
Croatia,Zagreb ,45.815011,15.981919
Cuba,Havana,23.05407,-82.345189
Curaçao,Willemstad,12.122422,-68.882423
Cyprus,Nicosia,35.185566,33.382276
Czech Republic,Prague,50.075538,14.4378
Denmark,Copenhagen,55.676097,12.568337
Djibouti,Djibouti,11.572077,43.145647
Dominica,Roseau,15.309168,-61.379355
Dominican Republic,Santo Domingo,18.486058,-69.931212
Ecuador,Quito,-0.180653,-78.467838
Egypt,Cairo,30.04442,31.235712
El Salvador,San Salvador,13.69294,-89.218191
Equatorial Guinea,Malabo,3.750412,8.737104
Eritrea,Asmara,15.322877,38.925052
Estonia,Tallinn,59.436961,24.753575
Ethiopia,Addis Ababa,8.980603,38.757761
Falkland Islands (Islas Malvinas),Stanley,-51.697713,-57.851663
Faroe Islands,Tórshavn,62.007864,-6.790982
Fiji,Suva,-18.124809,178.450079
Finland,Helsinki,60.173324,24.941025
France,Paris,48.856614,2.352222
French Guiana,Cayenne,4.92242,-52.313453
French Polynesia,Papeete,-17.551625,-149.558476
French Southern Territories,Saint-Pierre ,-21.3419,55.4778
Gabon,Libreville,0.416198,9.467268
Gambia,Banjul,13.454876,-16.579032
Georgia,Tbilisi,41.715138,44.827096
Germany,Berlin,52.520007,13.404954
Ghana,Accra,5.603717,-0.186964
Gibraltar,Gibraltar,36.140773,-5.353599
Greece,Athens,37.983917,23.72936
Greenland,Nuuk,64.18141,-51.694138
Grenada,Saint George's,12.056098,-61.7488
Guadeloupe,Basse-Terre,16.014453,-61.706411
Guam,Hagåtña,13.470891,144.751278
Guatemala,Guatemala City,14.634915,-90.506882
Guernsey,Saint Peter Port,49.455443,-2.536871
Guinea,Conakry,9.641185,-13.578401
Guinea-Bissau,Bissau,11.881655,-15.617794
Guyana,Georgetown,6.801279,-58.155125
Haiti,Port-au-Prince,18.594395,-72.307433
Honduras,Tegucigalpa,14.072275,-87.192136
Hong Kong,Hong Kong,22.396428,114.109497
Hungary,Budapest,47.497912,19.040235
Iceland,Reykjavík,64.126521,-21.817439
India,New Delhi,28.613939,77.209021
Indonesia,Jakarta,-6.208763,106.845599
Iran,Tehran,35.689198,51.388974
Iraq,Baghdad,33.312806,44.361488
Ireland,Dublin,53.349805,-6.26031
Isle of Man,Douglas,54.152337,-4.486123
Israel,Tel Aviv,32.0853,34.781768
Italy,Rome,41.902784,12.496366
Jamaica,Kingston,18.042327,-76.802893
Japan,Tokyo,35.709026,139.731992
Jersey,Saint Helier,49.186823,-2.106568
Jordan,Amman,31.956578,35.945695
Kazakhstan,Astana,51.160523,71.470356
Kenya,Nairobi,-1.292066,36.821946
Kiribati,Tarawa Atoll,1.451817,172.971662
Kosovo,Pristina,42.662914,21.165503
Kuwait,Kuwait City,29.375859,47.977405
Kyrgyzstan,Bishkek,42.874621,74.569762
Laos,Vientiane,17.975706,102.633104
Latvia,Riga,56.949649,24.105186
Lebanon,Beirut,33.888629,35.495479
Lesotho,Maseru,-29.363219,27.51436
Liberia,Monrovia,6.290743,-10.760524
Libya,Tripoli,32.887209,13.191338
Liechtenstein,Vaduz,47.14103,9.520928
Lithuania,Vilnius,54.687156,25.279651
Luxembourg,Luxembourg,49.611621,6.131935
Macau,Macau,22.166667,113.55
North Macedonia,Skopje,41.997346,21.427996
Madagascar,Antananarivo,-18.87919,47.507905
Malawi,Lilongwe,-13.962612,33.774119
Malaysia,Kuala Lumpur,3.139003,101.686855
Maldives,Malé,4.175496,73.509347
Mali,Bamako,12.639232,-8.002889
Malta,Valletta,35.898909,14.514553
Marshall Islands,Majuro,7.116421,171.185774
Martinique,Fort-de-France,14.616065,-61.05878
Mauritania,Nouakchott,18.07353,-15.958237
Mauritius,Port Louis,-20.166896,57.502332
Mayotte,Mamoudzou,-12.780949,45.227872
Mexico,Mexico City,19.432608,-99.133208
Micronesia,Palikir,6.914712,158.161027
Moldova,Chisinau,47.010453,28.86381
Monaco,Monaco,43.737411,7.420816
Mongolia,Ulaanbaatar,47.886399,106.905744
Montenegro,Podgorica,42.43042,19.259364
Montserrat,Plymouth,16.706523,-62.215738
Morocco,Rabat,33.97159,-6.849813
Mozambique,Maputo,-25.891968,32.605135
Myanmar,Naypyidaw,19.763306,96.07851
Nagorno-Karabakh Republic,Stepanakert,39.826385,46.763595
Namibia,Windhoek,-22.560881,17.065755
Nauru,Yaren,-0.546686,166.921091
Nepal,Kathmandu,27.717245,85.323961
Netherlands,Amsterdam,52.370216,4.895168
Netherlands Antilles,Willemstad ,12.1091242,-68.9316546
New Caledonia,Nouméa,-22.255823,166.450524
New Zealand,Wellington,-41.28646,174.776236
Nicaragua,Managua,12.114993,-86.236174
Niger,Niamey,13.511596,2.125385
Nigeria,Abuja,9.076479,7.398574
Niue,Alofi,-19.055371,-169.917871
Norfolk Island,Kingston,-29.056394,167.959588
North Korea,Pyongyang,39.039219,125.762524
Northern Cyprus,Nicosia,35.185566,33.382276
Northern Mariana Islands,Saipan,15.177801,145.750967
Norway,Oslo,59.913869,10.752245
Oman,Muscat,23.58589,58.405923
Pakistan,Islamabad,33.729388,73.093146
Palau,Ngerulmud,7.500384,134.624289
Palestine,Ramallah,31.9073509,35.5354719
Panama,Panama City,9.101179,-79.402864
Papua New Guinea,Port Moresby,-9.4438,147.180267
Paraguay,Asuncion,-25.26374,-57.575926
Peru,Lima,-12.046374,-77.042793
Philippines,Manila,14.599512,120.98422
Pitcairn Islands,Adamstown,-25.06629,-130.100464
Poland,Warsaw,52.229676,21.012229
Portugal,Lisbon,38.722252,-9.139337
Puerto Rico,San Juan,18.466334,-66.105722
Qatar,Doha,25.285447,51.53104
Réunion,Saint-Denis,-20.882057,55.450675
Romania,Bucharest,44.426767,26.102538
Russia,Moscow,55.755826,37.6173
Rwanda,Kigali,-1.957875,30.112735
Saint Pierre and Miquelon,Saint Pierre,46.775846,-56.180636
Saint Vincent and the Grenadines,Kingstown,13.160025,-61.224816
Samoa,Apia,-13.850696,-171.751355
San Marino,San Marino,43.935591,12.447281
São Tomé and Príncipe,São Tomé,0.330192,6.733343
Saudi Arabia,Riyadh,24.749403,46.902838
Senegal,Dakar,14.764504,-17.366029
Serbia,Belgrade,44.786568,20.448922
Seychelles,Victoria,-4.619143,55.451315
Sierra Leone,Freetown,8.465677,-13.231722
Singapore,Singapore,1.280095,103.850949
Slovakia,Bratislava,48.145892,17.107137
Slovenia,Ljubljana,46.056947,14.505751
Solomon Islands,Honiara,-9.445638,159.9729
Somalia,Mogadishu,2.046934,45.318162
South Africa,Pretoria,-25.747868,28.229271
South Georgia and the South Sandwich Islands,King Edward Point,-54.28325,-36.493735
South Korea,Seoul,37.566535,126.977969
South Ossetia,Tskhinvali,42.22146,43.964405
South Sudan,Juba,4.859363,31.57125
Spain,Madrid,40.416775,-3.70379
Sri Lanka,Sri Jayawardenepura Kotte,6.89407,79.902478
Saint Barthélemy,Gustavia,17.896435,-62.852201
Saint Kitts and Nevis,Basseterre,17.302606,-62.717692
Saint Lucia,Castries,14.010109,-60.987469
Saint Martin,Marigot,18.067519,-63.082466
Sudan,Khartoum,15.500654,32.559899
Suriname,Paramaribo,5.852036,-55.203828
Svalbard and Jan Mayen,Longyearbyen ,78.062,22.055
Eswatini,Mbabane,-26.305448,31.136672
Sweden,Stockholm,59.329323,18.068581
Switzerland,Bern,46.947974,7.447447
Syria,Damascus,33.513807,36.276528
Taiwan,Taipei,25.032969,121.565418
Tajikistan,Dushanbe,38.559772,68.787038
Tanzania,Dodoma,-6.162959,35.751607
Thailand,Bangkok,13.756331,100.501765
Timor-Leste,Dili,-8.556856,125.560314
Togo,Lomé,6.172497,1.231362
Tokelau,Nukunonu,-9.2005,-171.848
Tonga,Nukuʻalofa,-21.139342,-175.204947
Transnistria,Tiraspol,46.848185,29.596805
Trinidad and Tobago,Port of Spain,10.654901,-61.501926
Tristan da Cunha,Edinburgh of the Seven Seas,-37.068042,-12.311315
Tunisia,Tunis,36.806495,10.181532
Turkey,Ankara,39.933364,32.859742
Turkmenistan,Ashgabat,37.960077,58.326063
Turks and Caicos Islands,Cockburn Town,21.467458,-71.13891
Tuvalu,Funafuti,-8.520066,179.198128
U.S. Virgin Islands,Charlotte Amalie,18.3419,-64.930701
Uganda,Kampala,0.347596,32.58252
Ukraine,Kiev,50.4501,30.5234
United Arab Emirates,Abu Dhabi,24.299174,54.697277
United Kingdom,London,51.507351,-0.127758
United States,Washington,38.907192,-77.036871
Uruguay,Montevideo,-34.901113,-56.164531
Uzbekistan,Tashkent,41.299496,69.240073
Vanuatu,Port Vila,-17.733251,168.327325
Vatican City,Vatican City,41.902179,12.453601
Venezuela,Caracas,10.480594,-66.903606
Vietnam,Hanoi,21.027764,105.83416
Wallis and Futuna,Mata-Utu,-13.282509,-176.176447
Western Sahara,El Aaiún,27.125287,-13.1625
Yemen,Sana'a,15.369445,44.191007
Zambia,Lusaka,-15.387526,28.322817
Zimbabwe,Harare,-17.825166,31.03351'''

create_file()

