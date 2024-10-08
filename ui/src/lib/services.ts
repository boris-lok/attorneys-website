import type { Member, MemberDetail, SimpleMember } from '$lib/models/Member';
import type { CreateHomeRequest, HomeData, UpdateHomeRequest } from '$lib/models/Home';
import type { Language } from '$lib/models/Language';
import { from, of } from 'rxjs';
import type { Service } from '$lib/models/Services';
import type { SimpleArticle } from '$lib/models/Articles';

const BASE_URL = 'http://localhost:1234/api/v1';
const ADMIN_URL = `${BASE_URL}/admin`;
const TIMEOUT = 5000;

export const Members = {
	create: (member: Member, lang: Language) => {
		let data = JSON.stringify({
			...member,
			language: lang
		});

		const request: Promise<{ member_id: string }> = fetch(
			`${ADMIN_URL}/members`,
			{
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: data,
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json());

		return from(request);
	},
	uploadAvatar: (memberId: string, file: File) => {
		const formData = new FormData();
		formData.append('avatar', file);

		const request: Promise<Response> = fetch(
			`${ADMIN_URL}/members/${memberId}/avatar`,
			{
				method: 'POST',
				body: formData
			}
		);

		return from(request);
	},
	list: () => {
		const data: SimpleMember[] = [
			{
				member_id: '1',
				name: '蕭嘉豪'
			}, {
				member_id: '2',
				name: '陳致璇'
			}, {
				member_id: '3',
				name: '王筱雯'
			}
		];

		return of(data);
	},
	get: (memberId: string) => {
		const data: MemberDetail = {
			member_id: '1',
			name: 'Johnathan Doe',
			content: `
**Specialization**: Corporate Law  
**Years of Experience**: 12  
**Firm**: Doe & Associates  

## Contact Information
- **Email**: [johnathan.doe@example.com](mailto:johnathan.doe@example.com)
- **Phone**: +1-555-123-4567
- **Address**:  
  123 Legal Lane,  
  New York, NY 10001

## Education
- **Juris Doctor (JD)**  
  Harvard Law School, 2010
- **Bachelor of Arts in Political Science**  
  Yale University, 2006

## Bar Admissions
- **New York**: 2011
- **California**: 2013

## Practice Areas
- Mergers & Acquisitions
- Corporate Governance
- Securities Law
- Contract Negotiation

## Professional Affiliations
- American Bar Association
- New York State Bar Association
- Corporate Law Society

## Notable Cases
- **Acme Corp vs Global Tech (2018)**  
  Represented Acme Corp in a high-profile merger case, securing a favorable settlement.
  
- **Smith Industries Acquisition (2020)**  
  Led legal efforts in Smith Industries' acquisition by a major multinational corporation.

## Awards
- **Top Corporate Lawyer (2019)**  
  Awarded by Law Journal.
  
- **Rising Star in M&A Law (2016)**  
  Recognized by Legal 500.

## Languages Spoken
- English
- Spanish	
			`
		};

		return of(data);
	}
};

export const Home = {
	list: (language: Language) => {
		const request = fetch(
			`${BASE_URL}/home`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					'Accept-Language': language
				},
				signal: AbortSignal.timeout(TIMEOUT)
			}
		).then((res) => res.json())
			.then((json: Object) => {
				const data = 'home' in json ? json.home as HomeData[] : [];
				if (data.length === 0) {
					return null;
				}
				return data[0];
			});

		return from(request);
	},
	retrieve: (id: string, language: Language) => {
		const request = fetch(`${BASE_URL}/home/${id}`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				'Accept-Language': language
			},
			signal: AbortSignal.timeout(TIMEOUT)
		})
			.then(res => res.json())
			.then(res => 'home' in res ? res.home as HomeData : null);

		return from(request);
	},
	save: (req: CreateHomeRequest | UpdateHomeRequest) => {
		let method = 'POST';
		if ('id' in req) {
			method = 'PUT';
		}

		const request = fetch(
			`${ADMIN_URL}/home`,
			{
				method: method,
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(req),
				signal: AbortSignal.timeout(TIMEOUT)
			}
		);

		return from(request);
	}
};

export const Services = {
	list: () => {
		const data: Service[] = [{
			title: '訴訟及爭端處理',
			content: `- 民事、刑事訴訟
- 行政訴訟 / 國家賠償
- 促參、政府採購訴訟/調解
- 強制執行`
		}, {
			title: '非訟事件',
			content: `- 盡職調查Due Diligent
- 各式合約撰寫／翻譯
- 見證服務
- 公司設立登記
            `
		}, {
			title: '智慧財產權保護',
			content: `- 商標申請、商標異議、陳述意見及相關訴訟
- 著作權維權、
- 公平交易案件
- 國際網域名稱保護
- 個人資料保護案件
- 專利檢索、專利申請、專利侵害分析報告、公司智財輔導`
		}
			, {
				title: '公司及投資',
				content: `- 證券及資本市場、保險法
- 促參案件、工程及政府採購（含軍事採購）
- 銀行與融資
- 企業合併及股權收購
- 稅務案件
- 不動產／營建案件
- 勞資糾紛`
			}
		];

		return of(data);
	}
};

export const Articles = {
	list: () => {
		const data: SimpleArticle[] = [
			{
				'id': 'ART001',
				'title': 'The Future of Quantum Computing: Breaking Boundaries'
			},
			{
				'id': 'ART002',
				'title': '5 Unexpected Ways AI is Changing Healthcare'
			},
			{
				'id': 'ART003',
				'title': 'From Zero to Hero: How Startups are Disrupting Industries'
			},
			{
				'id': 'ART004',
				'title': 'Sustainable Tech: Innovations to Save the Planet'
			},
			{
				'id': 'ART005',
				'title': 'The Rise of Electric Vehicles: A New Era of Transportation'
			},
			{
				'id': 'ART006',
				'title': 'Mastering Remote Work: Productivity Tips for the Digital Nomad'
			},
			{
				'id': 'ART007',
				'title': 'Blockchain Beyond Cryptocurrency: Revolutionizing Supply Chains'
			},
			{
				'id': 'ART008',
				'title': 'Space Tourism: Exploring the Final Frontier in Luxury'
			},
			{
				'id': 'ART009',
				'title': 'The Ultimate Guide to Cybersecurity in 2024'
			},
			{
				'id': 'ART010',
				'title': 'How 5G Will Change the Way We Live and Work'
			}
		];

		return of(data);
	},
	get: (id: string) => {
		const article = {
			'id': 'ART012',
			'title': 'Exploring the Role of Renewable Energy in Modern Cities',
			'content': 'Renewable energy is rapidly transforming the landscape of urban environments. As cities grow and the demand for energy increases, reliance on sustainable energy sources like solar, wind, and hydropower is becoming critical. Modern cities are adopting renewable energy not only to meet the growing energy demand but also to reduce their carbon footprint and combat climate change.\n\n## Solar Power: Lighting Up the Future\nSolar panels are increasingly seen on rooftops and integrated into the infrastructure of buildings. With advancements in solar technology, cities are now able to harness more energy from the sun than ever before. In fact, many cities are aiming for **net-zero** energy consumption, where the amount of energy consumed is offset by the amount generated from renewable sources.\n\n## Wind Energy: Powering City Grids\nWind turbines are no longer limited to rural areas and offshore locations. Urban wind farms are emerging as a viable option for generating energy in cities. These turbines can be installed in open spaces, along coastlines, or even integrated into the architectural design of buildings. Wind energy, in combination with other renewable sources, plays a significant role in reducing the reliance on fossil fuels.\n\n## Challenges and Future Outlook\nWhile the shift to renewable energy is promising, it is not without its challenges. **Storage** and **distribution** remain two of the most significant hurdles. Innovations in battery technology and smart grids are helping to address these issues, but widespread adoption will take time. Despite these challenges, the future of renewable energy in cities is bright, with ongoing advancements pushing the boundaries of what’s possible.\n\nIn conclusion, renewable energy will be a cornerstone of urban development in the coming decades. Cities that invest in sustainable energy infrastructure today will reap the benefits of a cleaner, more sustainable tomorrow.'
		};
		return of(article);
	}
};