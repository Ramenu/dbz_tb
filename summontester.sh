#!/bin/bash

summon_min=0
summon_max=100
cards_per_multi=10
total_cards_summoned=$(($summon_max * $cards_per_multi))

num_r_found=0
num_sr_found=0
num_ssr_found=0

cd ./tb_web/src

for (( i="$summon_min"; i<"$summon_max"; ++i ))
do
	found=$(node ./Summon.js | awk '
BEGIN {
	r=0
	sr=0
	ssr=0
}

NF == 2 {
	if ($2 == "R")
		r += 1
	else if ($2 == "SR")
		sr += 1
	else if ($2 == "SSR")
		ssr += 1


}

END {
	printf "R: %s SR: %s SSR: %s", r, sr, ssr
}')

num_r_found=$(($num_r_found + $(echo $found | awk '{print $2}')))
num_sr_found=$(($num_sr_found + $(echo $found | awk '{print $4}')))
num_ssr_found=$(($num_ssr_found + $(echo $found | awk '{print $6}')))
done

cards_summoned=$(($num_r_found + $num_sr_found + $num_ssr_found))
if [[ "$cards_summoned" -eq "$total_cards_summoned" ]] 
then
	r_percentage=$(bc -l <<< 'scale=2;'"$num_r_found"/"$total_cards_summoned"*100)
	sr_percentage=$(bc -l <<< 'scale=2;'"$num_sr_found"/"$total_cards_summoned"*100)
	ssr_percentage=$(bc -l <<< 'scale=2;'"$num_ssr_found"/"$total_cards_summoned"*100)
	echo Percentage of R\'s found: "$r_percentage"%
	echo Percentage of SR\'s found: "$sr_percentage"%
	echo Percentage of SSR\'s found: "$ssr_percentage"%
else
	echo Fail!
fi

